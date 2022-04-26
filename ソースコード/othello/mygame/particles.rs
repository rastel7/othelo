use ggez::{graphics, Context, GameResult};
use glam::*;
use mint::Point2;
use rand::Rng;
use std::collections::VecDeque;
/*
    パーティクルの集合を管理
    パーティクルの削除を効率良く行うためにキューを2つもち，出し入れする
    LinkedListが理想だが，要素の削除がunsafeなため見送り
*/
pub struct Particles {
    queue1: VecDeque<Particle>,
    queue2: VecDeque<Particle>,
}
impl Particles {
    pub fn new() -> Particles {
        let it = Particles {
            queue1: VecDeque::new(),
            queue2: VecDeque::new(),
        };
        it
    }
    pub fn particle_count(&self) -> u32 {
        return (self.queue1.len() + self.queue2.len()) as u32;
    }
    fn update_queue(
        queue: &mut VecDeque<Particle>,
        nextqueue: &mut VecDeque<Particle>,
        delta: f32,
    ) {
        //現在使用しているキューが空になるまで更新を続ける
        while !queue.is_empty() {
            let mut it = queue.pop_front().unwrap();
            //パーティクルが持つ減少度と前フレームからの経過時間より，各パラメータの減少量を決定
            let diff = 1.0 - it.attenuation * delta;
            let alpha = it.color.a * diff;
            if alpha <= 0.15 {
                //透明度が一定以下になったパーティクルは削除
                continue;
            }
            it.scale *= diff;
            it.position.x += it.speed.x * diff;
            it.position.y += it.speed.y * diff;
            it.color = graphics::Color::new(it.color.r, it.color.g, it.color.b, alpha);
            let new_acceleration = Point2 {
                x: it.acceleration.x - (it.acceleration.x / 5.0 * diff),
                y: it.acceleration.y - (it.acceleration.y / 5.0 * diff),
            };
            it.acceleration = new_acceleration;
            let new_speed = Point2 {
                x: it.speed.x + it.acceleration.x * diff,
                y: it.speed.y + it.acceleration.y * diff,
            };
            it.speed = new_speed;
            //更新が終わった要素は，次に使用するキューへと入れる
            nextqueue.push_back(it);
        }
    }
    pub fn update(&mut self, delta: f32) -> GameResult<()> {
        //パーティクルが入っているキューを実際に更新を行うupdate_queue関数へと渡す
        if self.queue1.len() != 0 {
            Particles::update_queue(&mut self.queue1, &mut self.queue2, delta);
        } else {
            Particles::update_queue(&mut self.queue2, &mut self.queue1, delta);
        };
        Ok(())
    }
    fn draw_queue(
        queue: &mut VecDeque<Particle>,
        nextqueue: &mut VecDeque<Particle>,
        ctx: &mut Context,
    ) {
        while !queue.is_empty() {
            let it = queue.pop_front().unwrap();
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(0.0, 0.0),
                it.scale,
                0.1,
                it.color,
            )
            .unwrap();
            let _ = graphics::draw(ctx, &circle, (it.position,));
            nextqueue.push_back(it);
        }
    }
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //パーティクルが入っているキューを実際に更新を行うupdate_queue関数へと渡す
        if self.queue1.len() != 0 {
            Particles::draw_queue(&mut self.queue1, &mut self.queue2, ctx);
        } else {
            Particles::draw_queue(&mut self.queue2, &mut self.queue1, ctx);
        };
        Ok(())
    }
    fn add_particle(&mut self, it: Particle) {
        //値が存在するキューへ追加する
        if self.queue1.len() != 0 {
            self.queue1.push_back(it);
        } else {
            self.queue2.push_back(it);
        };
    }
    /*
        石が置かれたorひっくり返った時に発生するパーティクルを生成
    */
    pub fn create_stone_particle(&mut self, pos: Point2<f32>, num: usize, color: &super::Stone) {
        let mut random = rand::thread_rng();
        for _ in 0..num {
            let color = match color {
                super::Stone::Black => 0.0,
                super::Stone::White => 1.0,
                _ => 0.5,
            };
            let color = graphics::Color::new(color, color, color, random.gen_range(0.7, 1.0));
            let speed = random.gen_range(-5.0, 5.0); //パーティクルの速度(スカラ)を設定
            let dir = Point2 {
                x: random.gen_range(-1.0, 1.0) as f32,
                y: random.gen_range(-1.0, 1.0) as f32,
            };
            let acceleration = Point2 {
                x: -1.0 * speed * dir.x / (dir.x * dir.x + dir.y * dir.y).sqrt() * 0.16,
                y: -1.0 * speed * dir.y / (dir.x * dir.x + dir.y * dir.y).sqrt() * 0.16,
            };
            let dir = Point2 {
                x: speed * dir.x / (dir.x * dir.x + dir.y * dir.y).sqrt(),
                y: speed * dir.y / (dir.x * dir.x + dir.y * dir.y).sqrt(),
            };
            let it = Particle::new(
                pos,
                dir,
                random.gen_range(5.0, 8.0),
                random.gen_range(2.0, 5.0),
                acceleration,
                color,
            );
            self.add_particle(it);
        }
    }
    /*
        ゲーム終了時用の紙吹雪の作成
        終了時にユーザーが勝っていた場合はパーティクルを左右から1000個ずつ発生させる
    */
    pub fn create_confetti(&mut self) {
        let color_range = 0.5;
        let count = 2000;
        let y_pos = 100.0;
        let mut random = rand::thread_rng();
        for i in 0..count {
            let position = if i % 2 == 0 {
                Point2 { x: 0.0, y: y_pos }
            } else {
                Point2 {
                    x: crate::WIDTH,
                    y: y_pos,
                }
            };
            let scale = random.gen_range(10.0, 20.0);
            let speed = Point2 {
                x: random.gen_range(1.0, 6.0) * if i % 2 == 0 { 1.0 } else { -1.0 },
                y: -1.0 * random.gen_range(10.0, 12.0),
            };
            let attenuation = random.gen_range(1.0, 3.0);
            let acceleration = random.gen_range(3.0, 6.0);
            let color = graphics::Color::new(
                random.gen_range(color_range, 1.0),
                random.gen_range(color_range, 1.0),
                random.gen_range(color_range, 1.0),
                1.0,
            );
            let particle = Particle::new(
                position,
                speed,
                scale,
                attenuation,
                Point2 {
                    x: 0.0,
                    y: 1.0 * acceleration,
                },
                color,
            );
            self.add_particle(particle);
        }
    }
}
/*
    パーティクル一つ分を表現
*/
struct Particle {
    position: Point2<f32>,
    speed: Point2<f32>,
    scale: f32,
    attenuation: f32,
    acceleration: Point2<f32>,
    color: graphics::Color,
}
impl Particle {
    pub fn new(
        position: Point2<f32>,
        speed: Point2<f32>,
        scale: f32,
        attention: f32,
        acceleration: Point2<f32>,
        color: graphics::Color,
    ) -> Particle {
        Particle {
            position: position,
            speed: speed,
            scale: scale,
            attenuation: attention,
            acceleration: acceleration,
            color: color,
        }
    }
}
