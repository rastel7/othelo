use crate::mygame::{predict, Stone, BOARDSIZE};
use ggez::{graphics, Context, GameResult};
use glam::*;
use mint::Point2;
#[derive(Clone, Copy)]
pub struct Board {
    pub gamebord: [[Stone; BOARDSIZE]; BOARDSIZE],
    size: usize,
    pub black_num: u32,
    pub white_num: u32,
    window_width: u32,
    window_height: u32,
}
impl Board {
    /*初期宣言 */
    pub fn new(size: usize, width: u32, height: u32) -> Board {
        let mut v: [[Stone; BOARDSIZE]; BOARDSIZE] = [[Stone::Blank; BOARDSIZE]; BOARDSIZE];
        v[size / 2][size / 2] = Stone::White;
        v[size / 2][size / 2 - 1] = Stone::Black;
        v[size / 2 - 1][size / 2 - 1] = Stone::White;
        v[size / 2 - 1][size / 2] = Stone::Black;
        Board {
            gamebord: v,
            size: size,
            black_num: 0,
            white_num: 0,
            window_width: width,
            window_height: height,
        }
    }
    /*
        盤面上の石と盤面の枠を描写
    */
    pub fn draw(
        &mut self,
        ctx: &mut Context,
        rotationrecord: &[[f32; BOARDSIZE]; BOARDSIZE],
    ) -> GameResult<()> {
        //1マスの大きさ
        let cell_size: u32 =
            std::cmp::min(self.window_width, self.window_height) / self.size as u32;
        //各コマの描画
        for y in 0..self.size {
            for x in 0..self.size {
                if rotationrecord[y][x] == 0.0 {
                    match self.gamebord[y][x] {
                        Stone::Blank => {}
                        _ => {
                            //設置済みの石の描写
                            self.draw_stone(ctx, cell_size as f32, (x, y), &self.gamebord[y][x])?;
                        }
                    };
                } else {
                    match self.gamebord[y][x] {
                        Stone::Blank => {}
                        _ => {
                            //回転中の石の描写
                            self.draw_rotating_stone(
                                ctx,
                                cell_size as f32,
                                (x, y),
                                &self.gamebord[y][x],
                                rotationrecord[y][x],
                            )?;
                        }
                    };
                }
            }
        }
        //枠の描写
        self.draw_frame(ctx, cell_size as f32)?;
        Ok(())
    }
    fn draw_rotating_stone(
        &self,
        ctx: &mut Context,
        cell_size: f32,
        pos: (usize, usize),
        stone: &Stone,
        rotate_position: f32,
    ) -> GameResult<()> {
        /*
            現在回転中，もしくはこれから回転する石の描写
        */
        let pos = mint::Point2 {
            x: pos.0 as f32 * cell_size + cell_size / 2.0,
            y: pos.1 as f32 * cell_size + cell_size / 2.0,
        };
        if 0.5 <= rotate_position && rotate_position <= 1.0 {
            /*
            回転中に石が縦向きになっているのを表現するために，
            下側に反対色を描画
            */
            let color = match stone.return_reverse_color() {
                Stone::White => {
                    if rotate_position > 0.5 {
                        graphics::Color::BLACK
                    } else {
                        graphics::Color::WHITE
                    }
                }
                Stone::Black => {
                    if rotate_position > 0.5 {
                        graphics::Color::WHITE
                    } else {
                        graphics::Color::BLACK
                    }
                }
                _ => {
                    eprintln!("Blankを置こうとしています");
                    graphics::Color::new(0.0, 0.0, 0.0, 0.0)
                }
            };
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(0.0, 0.0),
                cell_size as f32 / 2.6,
                0.1,
                color,
            )?;
            let param = graphics::DrawParam::default()
                .dest(Point2 {
                    x: pos.x + 2.0,
                    y: pos.y,
                })
                .scale(Vec2::new(2.0 * (0.5 - rotate_position).abs(), 1.0));
            graphics::draw(ctx, &circle, param)?;
        }
        let color = match stone {
            Stone::White => {
                if rotate_position > 0.5 {
                    graphics::Color::BLACK
                } else {
                    graphics::Color::WHITE
                }
            }
            Stone::Black => {
                if rotate_position > 0.5 {
                    graphics::Color::WHITE
                } else {
                    graphics::Color::BLACK
                }
            }
            _ => {
                eprintln!("Blankを置こうとしています");
                graphics::Color::new(0.0, 0.0, 0.0, 0.0)
            }
        };
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            cell_size as f32 / 2.6,
            0.1,
            color,
        )?;
        let param = if rotate_position <= 1.0 {
            graphics::DrawParam::default()
                .dest(pos)
                .scale(Vec2::new(2.0 * (0.5 - rotate_position).abs(), 1.0))
        } else {
            graphics::DrawParam::default().dest(pos)
        };
        graphics::draw(ctx, &circle, param)?;
        Ok(())
    }
    fn draw_stone(
        &self,
        ctx: &mut Context,
        cell_size: f32,
        pos: (usize, usize),
        stone: &Stone,
    ) -> GameResult<()> {
        /*
            与えられたボード上座標に対して，与えられた色の石を描写する
        */
        let pos = mint::Point2 {
            x: pos.0 as f32 * cell_size + cell_size / 2.0,
            y: pos.1 as f32 * cell_size + cell_size / 2.0,
        };
        let stone = stone.return_reverse_color();
        let color = match stone {
            Stone::Black => graphics::Color::new(0.5, 0.5, 0.5, 0.2),
            Stone::White => graphics::Color::new(0.5, 0.5, 0.5, 0.2),
            Stone::Blank => {
                eprintln!("Blankを置こうとしています");
                graphics::Color::new(0.0, 0.0, 0.0, 0.0)
            }
        };
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            cell_size as f32 / 2.45,
            0.1,
            color,
        )?;
        graphics::draw(ctx, &circle, (pos,))?;
        let stone = stone.return_reverse_color();
        let color = match stone {
            Stone::Black => graphics::Color::BLACK,
            Stone::White => graphics::Color::WHITE,
            Stone::Blank => {
                eprintln!("Blankを置こうとしています");
                graphics::Color::new(0.0, 0.0, 0.0, 0.0)
            }
        };
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            cell_size as f32 / 2.6,
            0.1,
            color,
        )?;
        graphics::draw(ctx, &circle, (pos,))?;
        Ok(())
    }
    fn draw_frame(&self, ctx: &mut Context, cell_size: f32) -> GameResult<()> {
        /*
            各マスの枠を描画
        */
        let rect = graphics::Mesh::new_line(
            ctx,
            &[
                Vec2::new(0.0, 0.0),
                Vec2::new(
                    std::cmp::min(self.window_width, self.window_height) as f32,
                    0.,
                ),
            ],
            2.,
            graphics::Color::BLACK,
        )?;
        for x in 0..(self.size + 1) {
            let pos: Vec2 = Vec2::new(0.0, (x as f32 * cell_size) as f32);
            graphics::draw(ctx, &rect, (pos,))?;
        }
        let rect = graphics::Mesh::new_line(
            ctx,
            &[
                Vec2::new(0.0, 0.0),
                Vec2::new(
                    0.,
                    std::cmp::min(self.window_width, self.window_height) as f32,
                ),
            ],
            2.4,
            graphics::Color::BLACK,
        )?;
        for y in 0..(self.size + 1) {
            let pos: Vec2 = Vec2::new((y as f32 * cell_size) as f32, 0.0);
            graphics::draw(ctx, &rect, (pos,))?;
        }
        Ok(())
    }
    pub fn draw_candidate(&mut self, ctx: &mut Context, user: Stone) -> GameResult<()> {
        /*
            石をおける場所の候補を表示する
        */
        let cell_size: u32 =
            std::cmp::min(self.window_width, self.window_height) / self.size as u32;
        let can_pos_list = predict::can_set_pos(&self, user);
        let color = match user {
            Stone::Black => graphics::Color::new(0.0, 0.0, 0.0, 0.60),
            Stone::White => graphics::Color::new(1.0, 1.0, 1.0, 0.15),
            _ => graphics::Color::new(0.0, 0.0, 0.0, 0.0),
        };
        for pos in can_pos_list {
            let pos: Vec2 = Vec2::new(
                (pos.1 as f32 * cell_size as f32 + (cell_size as f32) / 2.) as f32,
                (pos.0 as f32 * cell_size as f32 + (cell_size as f32) / 2.) as f32,
            );
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(0.0, 0.0),
                cell_size as f32 / 2.6,
                0.1,
                color,
            )?;
            graphics::draw(ctx, &circle, (pos,))?;
        }
        Ok(())
    }
    pub fn draw_placed_rotation_stone(
        &self,
        ctx: &mut Context,
        candidate: &Vec<(usize, usize)>,
        rotating_counter: f32,
        cell_size: f32,
    ) -> GameResult<()> {
        /*
            プレイヤーが石を置ける場所にマウスオーバーしたときに，
            ひっくり返る石に対して○を描画して教える
        */
        let max_rotating_counter = 300.0;
        let update_cnt = 100.0;
        let max_scale = cell_size * 1.1;
        for pos in candidate {
            let mut rotating_counter = rotating_counter + update_cnt * 2.0;
            let pos: Vec2 = Vec2::new(
                (pos.1 as f32 * cell_size as f32 + (cell_size as f32) / 2.) as f32,
                (pos.0 as f32 * cell_size as f32 + (cell_size as f32) / 2.) as f32,
            );
            //大きい円ほど透明
            while rotating_counter > 1.0 {
                let color = graphics::Color::new(
                    1.0,
                    1.0,
                    1.0,
                    1.0 * (1.0 - (rotating_counter / max_rotating_counter)),
                );
                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::stroke(1.0),
                    Vec2::new(0.0, 0.0),
                    max_scale * (rotating_counter / max_rotating_counter),
                    0.1,
                    color,
                )?;
                graphics::draw(ctx, &circle, (pos,))?;
                let color = graphics::Color::new(0.3, 0.3, 1.0, color.a / 3.0);
                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::stroke(3.0),
                    Vec2::new(0.0, 0.0),
                    max_scale * (rotating_counter / max_rotating_counter),
                    0.1,
                    color,
                )?;
                graphics::draw(ctx, &circle, (pos,))?;
                rotating_counter -= update_cnt;
            }
        }
        Ok(())
    }
    pub fn calc_board_score(&mut self) -> f32 {
        self.white_num as f32 - self.black_num as f32
        //白からみた盤面のスコアを計算
    }
    pub fn return_win(&mut self) -> Stone {
        /*
            どちらの色が勝っているかを返す
        */
        self.count_stone();
        if self.white_num > self.black_num {
            Stone::White
        } else if self.white_num < self.black_num {
            Stone::Black
        } else {
            Stone::Blank
        }
    }
    pub fn count_stone(&mut self) {
        /*
            盤面上にある各色の数を更新する
        */
        let mut black = 0;
        let mut white = 0;
        for x in 0..self.gamebord.len() {
            for y in 0..self.gamebord[0].len() {
                match self.gamebord[y][x] {
                    Stone::White => white += 1,
                    Stone::Black => black += 1,
                    _ => (),
                }
            }
        }
        self.white_num = white;
        self.black_num = black;
    }
    pub fn boardcoordinate_to_screencoordinate(&self, pos: (usize, usize)) -> Point2<f32> {
        /*
            盤面座標を画面座標へと変換する
        */
        let cell_size: f32 =
            std::cmp::min(self.window_width, self.window_height) as f32 / self.size as f32;
        Point2 {
            x: pos.1 as f32 * cell_size + cell_size / 2.0,
            y: pos.0 as f32 * cell_size + cell_size / 2.0,
        }
    }
    pub fn screencoordinate_to_boardcoordinate(&self, pos: mint::Point2<f32>) -> Option<(u32, u32)> {
        /* 
            画面座標を盤面座標へと変換する
        */
        let mn = std::cmp::min(self.window_width, self.window_height);
        if mn <= pos.x as u32 || mn <= pos.y as u32 {
            return None;
        }
        let cell_size: u32 =
            std::cmp::min(self.window_width, self.window_height) / self.size as u32;
        let pos = ((pos.y as u32) / cell_size, (pos.x as u32) / cell_size);
        if std::cmp::max(pos.0, pos.1) >= super::BOARDSIZE as u32 {
            return None;
        }
        Some(pos)
    }
    pub fn setstone(&mut self, list: &Vec<(usize, usize)>, color: Stone) {
        for pos in list {
            self.gamebord[pos.0][pos.1] = color;
        }
    }
}
