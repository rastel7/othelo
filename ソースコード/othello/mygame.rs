use board::Board;
use ggez::event::EventHandler;
use ggez::{graphics, Context, GameResult};
use glam::*;
use stone::Stone;
use user::User;
mod board;
mod button;
mod detailedstatus;
mod mouse;
mod particles;
mod predict;
mod se;
mod stone;
mod user;
pub struct MyGame {
    pub board: board::Board,
    ui: button::UIs,
    particles: particles::Particles,
    rotationrecord: [[f32; BOARDSIZE]; BOARDSIZE],
    window_width: u32,
    window_height: u32,
    pub now_user: User,
    pub passed: bool,
    pub mouse_inf: mouse::MouseInf,
    font: graphics::Font,
    se: se::Se,
    status: detailedstatus::Status,
}
pub enum ButtonEventList {
    Reset,
    Pass,
}
const FONT_SIZE: f32 = 48.0;
const SMALL_FONT_SIZE: f32 = 32.0;
const USER_COLOR: Stone = Stone::White;
const CPU_COLOR: Stone = Stone::Black;
pub const BOARDSIZE: usize = 8;
impl MyGame {
    pub fn new(ctx: &mut Context, width: u32, height: u32) -> MyGame {
        let fontpass = "/NotoSansJP-Regular.otf";
        MyGame {
            board: board::Board::new(BOARDSIZE, width, height),
            ui: button::UIs::new(),
            particles: particles::Particles::new(),
            rotationrecord: [[0.0; BOARDSIZE]; BOARDSIZE],
            window_width: width,
            window_height: height,
            now_user: User { now: Stone::White },
            passed: false,
            mouse_inf: mouse::MouseInf {
                pos: mint::Point2 { x: 0.0, y: 0.0 },
                pressed: false,
            },
            font: graphics::Font::new(ctx, fontpass).unwrap(),
            se: se::Se::new(ctx),
            status: detailedstatus::Status::new(0.0),
        }
    }
    /*
        UIの表示
    */
    pub fn draw_ui(&mut self, ctx: &mut Context) -> GameResult<()> {
        {
            //プレイヤー側の石数と色を描画
            let mut text = self.board.white_num.to_string();
            if text.len() == 1 {
                text += " ";
            }
            let mut text: String = (text + " You").to_string();
            match self.now_user.now {
                Stone::White => text += " ←",
                _ => {}
            }
            let text = graphics::Text::new((text, self.font, FONT_SIZE));
            let _ = graphics::draw(
                ctx,
                &text,
                (Vec2::new(0., self.window_height as f32 - FONT_SIZE),),
            );
            let text = "White";
            let text = graphics::Text::new((text, self.font, SMALL_FONT_SIZE));
            let _ = graphics::draw(
                ctx,
                &text,
                (Vec2::new(
                    0.,
                    self.window_height as f32 - FONT_SIZE - SMALL_FONT_SIZE / 2.0,
                ),),
            );
        }
        {
            //CPU側の石の数と色を表示
            let mut text = match self.ret_nowuser() {
                Stone::Black => "→".to_string(),
                _ => "".to_string(),
            };
            text += "CPU";
            if self.board.black_num < 10 {
                text += " ";
            }
            text += &self.board.black_num.to_string();
            let text = graphics::Text::new(graphics::TextFragment {
                text: text,
                color: Some(graphics::Color::new(0.0, 0.0, 0.0, 1.0)),
                font: Some(self.font),
                scale: Some(graphics::PxScale::from(FONT_SIZE)),
            });
            let _ = graphics::draw(
                ctx,
                &text,
                (Vec2::new(
                    self.window_width as f32 - text.dimensions(ctx).w,
                    self.window_height as f32 - FONT_SIZE,
                ),),
            );
            let text = "Black".to_string();
            let text = graphics::Text::new(graphics::TextFragment {
                text: text,
                color: Some(graphics::Color::new(0.0, 0.0, 0.0, 1.0)),
                font: Some(self.font),
                scale: Some(graphics::PxScale::from(SMALL_FONT_SIZE)),
            });
            let _ = graphics::draw(
                ctx,
                &text,
                (Vec2::new(
                    self.window_width as f32 - text.dimensions(ctx).w,
                    self.window_height as f32 - FONT_SIZE - SMALL_FONT_SIZE / 2.0,
                ),),
            );
        }
        {
            //CPUが探索を行っている時，その旨を描画
            if self.status.thinking {
                let text = "CPU思考中・・・".to_string();
                let transparency = if self.status.get_blink_description() < 0.5 {
                    0.5 + self.status.get_blink_description()
                } else {
                    1.5 - self.status.get_blink_description()
                };
                let text = graphics::Text::new(graphics::TextFragment {
                    text: text,
                    color: Some(graphics::Color::new(0.9, 0.9, 0.9, transparency)),
                    font: Some(self.font),
                    scale: Some(graphics::PxScale::from(FONT_SIZE)),
                });
                graphics::draw(
                    ctx,
                    &text,
                    (Vec2::new(
                        (self.window_width as f32) / 2.0 - text.dimensions(ctx).w / 2.0,
                        (self.window_height as f32) / 1.3 - text.dimensions(ctx).h / 2.0,
                    ),),
                )?;
            }
        }
        //UI類の表示
        for it in self.ui.buttons.iter() {
            it.draw(ctx, &self.font, &self.mouse_inf, &self.board)?;
        }
        Ok(())
    }
    fn draw_can_rotate(&self, ctx: &mut Context) -> GameResult<()> {
        //現在ユーザー側のターンかつ置ける位置にマウスオーバーしている場合，ひっくり返る石を表示する
        if self.ret_nowuser() as i32 == Stone::White as i32 {
            match self
                .board
                .screencoordinate_to_boardcoordinate(self.mouse_inf.pos)
            {
                None => {}
                Some(v) => {
                    let cell_size: f32 = std::cmp::min(self.window_width, self.window_height)
                        as f32
                        / BOARDSIZE as f32;
                    self.board.draw_placed_rotation_stone(
                        ctx,
                        &predict::can_cnt(
                            &self.board.gamebord,
                            Stone::White,
                            (v.0 as usize, v.1 as usize),
                            false,
                        ),
                        self.status.get_placed_rotating_stone_count(),
                        cell_size,
                    )?;
                }
            }
        }
        Ok(())
    }
    /*
        ゲーム終了時にどちらが勝利したかを表示
    */
    fn draw_win_or_lose(&self, ctx: &mut Context) -> GameResult<()> {
        let (text, color, rectcolor) = if self.board.white_num > self.board.black_num {
            (
                "White \n Win!",
                graphics::Color::WHITE,
                graphics::Color::new(0.0, 0.0, 0.0, 0.6),
            )
        } else if self.board.black_num > self.board.white_num {
            (
                "Black \n Win!",
                graphics::Color::BLACK,
                graphics::Color::new(1.0, 1.0, 1.0, 0.6),
            )
        } else {
            (
                "Draw",
                graphics::Color::new(0.9, 0.9, 0.9, 1.0),
                graphics::Color::new(0.0, 0.0, 0.0, 0.6),
            )
        };
        let text = graphics::Text::new(graphics::TextFragment {
            text: text.to_string(),
            color: Some(color),
            font: Some(self.font),
            scale: Some(graphics::PxScale::from(FONT_SIZE * 2.0)),
        });
        let rect = graphics::Rect {
            x: -text.dimensions(ctx).w * 0.6,
            y: -text.dimensions(ctx).h * 0.6,
            w: text.dimensions(ctx).w * 1.2,
            h: text.dimensions(ctx).h * 1.2,
        };

        let rect = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, rectcolor)?;
        graphics::draw(
            ctx,
            &rect,
            (Vec2::new(
                (self.window_width as f32) / 2.0,
                (self.window_height as f32) / 2.0,
            ),),
        )?;
        graphics::draw(
            ctx,
            &text,
            (Vec2::new(
                (self.window_width as f32) / 2.0 - text.dimensions(ctx).w / 2.0,
                (self.window_height as f32) / 2.0 - text.dimensions(ctx).h / 2.0,
            ),),
        )?;
        Ok(())
    }
    //現在のターンがどちらの色かを返す
    pub fn ret_nowuser(&self) -> Stone {
        return self.now_user.now;
    }
    //ゲーム内容を初期化
    fn reset_game(&mut self) {
        self.board = board::Board::new(BOARDSIZE, self.window_width, self.window_height);
        self.now_user = User { now: Stone::White };
        self.passed = false;
        self.mouse_inf = mouse::MouseInf {
            pos: mint::Point2 { x: 0.0, y: 0.0 },
            pressed: false,
        };
        self.rotationrecord = [[0.0; BOARDSIZE]; BOARDSIZE];
        self.particles = particles::Particles::new();
        self.status.game_end = false;
    }
    //引数で与えられた石達を回転中or回転待機と設定する
    fn set_rotation_stone(&mut self, list: &Vec<(usize, usize)>) {
        let delay = 0.5; //遠くにある石はdelayの値だけひっくり返るのを遅らせる
        for i in 1..list.len() {
            let dist = (list[0].0 as i32 - list[i].0 as i32).abs()
                + (list[0].1 as i32 - list[i].1 as i32).abs();
            self.rotationrecord[list[i].0][list[i].1] = 1.0 + delay * dist as f32;
        }
        self.status.rotating = true;
    }
    //回転中or回転待機の石たちの回転角度をしめすパラメータを更新
    fn update_rotation_stone(&mut self, ctx: &Context) {
        self.status.rotating = false;
        for i in 0..self.rotationrecord.len() {
            for j in 0..self.rotationrecord[i].len() {
                /*
                    0<rotationrecord[i][j]<=1: 回転中
                    1.0<rotationrecord[i][j]: 回転待機中
                */
                let bounder = 0.5; //パーティクルを発生させる境界
                if self.rotationrecord[i][j] > bounder
                    && (self.rotationrecord[i][j] - ggez::timer::delta(ctx).as_secs_f32() * 4.0)
                        < bounder
                {
                    //角度がbouderをまたいだ瞬間に，ひっくり返ったことを表現するパーティクルを発生させる
                    self.particles.create_stone_particle(
                        self.board.boardcoordinate_to_screencoordinate((i, j)),
                        100,
                        &self.board.gamebord[i][j],
                    );
                }
                //経過した時間に応じて回転角度を変更していく
                self.rotationrecord[i][j] =
                    self.rotationrecord[i][j] - ggez::timer::delta(ctx).as_secs_f32() * 4.0;
                if self.rotationrecord[i][j] < 0.0 {
                    self.rotationrecord[i][j] = 0.0;
                } else {
                    self.status.rotating = true;
                }
            }
        }
    }
    //ゲームが終了しているか否かを返す
    pub fn is_gameend(&self) -> bool {
        predict::can_set_pos(&self.board, Stone::Black).len() == 0
            && predict::can_set_pos(&self.board, Stone::White).len() == 0
            && !self.status.rotating
    }
    fn cpu_set_stone(&mut self, ctx: &mut Context, stone: Stone) -> Option<GameResult<()>> {
        //None:現在のターン側の色ではない
        //Some(Ok):現在のターン　これが終わったらupdate関数はreturnを行う
        if self.ret_nowuser() as i32 == stone as i32 {
            let pos = predict::montecarlo::montecarlotree(&self.board, stone, &self.status);
            if self.status.rotating {
                return Some(Ok(())); //石の回転中は停止
            }
            match pos {
                Some(p) => {
                    match p {
                        None => {} //置ける場所無し
                        Some(w) => {
                            //wに置くという計算結果
                            let list = predict::can_cnt(&self.board.gamebord, stone, w, false);
                            self.board.setstone(&list, stone);
                            self.set_rotation_stone(&list);
                            self.particles.create_stone_particle(
                                self.board.boardcoordinate_to_screencoordinate(w),
                                25,
                                &stone,
                            );
                            self.se.play_stone(ctx).unwrap();
                        }
                    }
                    self.status.thinking = false;
                    self.now_user.nextuser();
                }
                None => {
                    //現在思考中
                    self.status.thinking = true;
                }
            };
            self.mouse_inf.set_mouseinf(ctx);
            self.board.count_stone();
            return Some(Ok(()));
        }
        None
    }
    fn player_set_stone(&mut self, ctx: &mut Context, stone: Stone) -> Option<GameResult<()>> {
        //None:現在のターン側の色ではない
        //Some(Ok):現在のターン　これが終わったらupdate関数はreturnを行う
        if self.mouse_inf.is_clicked(ctx) && self.ret_nowuser() as i32 == stone as i32 {
            match self
                .board
                .screencoordinate_to_boardcoordinate(ggez::input::mouse::position(ctx))
            {
                Some(t) => {
                    let list = predict::can_cnt(
                        &self.board.gamebord,
                        self.ret_nowuser(),
                        (t.0 as usize, t.1 as usize),
                        false,
                    );
                    /*
                        盤面をコンソール上に表示
                    */
                    for i in self.board.gamebord.iter() {
                        println!("{:?}", i);
                    }
                    println!("");
                    if list.len() > 0 {
                        let _ = self.board.setstone(&list, self.ret_nowuser());
                        self.now_user.nextuser();
                        self.set_rotation_stone(&list);
                        self.particles.create_stone_particle(
                            self.board
                                .boardcoordinate_to_screencoordinate((t.0 as usize, t.1 as usize)),
                            25,
                            &stone,
                        );
                        self.se.play_stone(ctx).unwrap();
                    }
                }
                None => {}
            }
        }
        None
    }
}
impl EventHandler for MyGame {
    /*
        毎フレームの最初に呼び出される
    */
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        //文字の点滅と，ひっくり返る石候補の提示に使う値の更新
        self.status
            .update_status(ggez::timer::delta(ctx).as_secs_f32() * 1.0)?;
        //パーティクルステータスの更新
        self.particles
            .update(ggez::timer::delta(ctx).as_secs_f32())?;
        let title = "MyOthelloGame".to_string();
        graphics::set_window_title(ctx, &title);
        //回転中の石ステータスの更新
        self.update_rotation_stone(ctx);
        //ボタン類の処理
        let mut buttoneventlist: Vec<ButtonEventList> = Vec::new();
        for it in self.ui.buttons.iter() {
            //押されていたらイベントリストに対応するイベントを記録
            match it.clicked(&self.mouse_inf, ctx) {
                Some(true) => buttoneventlist.push(it.action().unwrap()),
                _ => {}
            };
        }
        //ボタンによって登録されたイベントリストを実行
        for event in buttoneventlist {
            match event {
                ButtonEventList::Reset => {
                    self.se.play_button(ctx)?;
                    self.reset_game();
                }
                ButtonEventList::Pass => {
                    //パスできるのは，プレイヤー側のターンかつおける場所がなかったときのみ
                    if self.now_user.now as i32 == USER_COLOR as i32
                        && predict::can_set_pos(&self.board, USER_COLOR).len() == 0
                        && !self.is_gameend()
                    {
                        self.se.play_button(ctx)?;
                        self.now_user.nextuser();
                        self.mouse_inf.set_mouseinf(ctx);
                        return Ok(());
                    }
                }
            };
        }
        //プレイヤー側の石置き判定　CPUが思考中だった場合はreturn
        match self.cpu_set_stone(ctx, CPU_COLOR) {
            None => {}
            Some(_) => {
                return Ok(());
            }
        };
        //プレイヤー側の石置き判定
        match self.player_set_stone(ctx, USER_COLOR) {
            None => {}
            Some(_) => {
                return Ok(());
            }
        };
        self.mouse_inf.set_mouseinf(ctx);
        self.board.count_stone();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0.05, 0.3, 0.05, 1.));
        /*
            ゲーム中はパーティクルはコマの下へ描画
            ゲームが終わったら優先度を上げる
        */
        if !self.is_gameend() {
            self.particles.draw(ctx)?;
        }
        //枠と石の描画
        self.board.draw(ctx, &self.rotationrecord)?;
        //置ける候補の描画
        self.board.draw_candidate(ctx, self.now_user.now)?;
        self.draw_can_rotate(ctx)?;
        self.draw_ui(ctx)?;
        if self.is_gameend() {
            if !self.status.game_end && self.board.return_win() as i32 == USER_COLOR as i32 {
                self.particles.create_confetti();
            }
            self.status.game_end = true;
            self.particles.draw(ctx)?;
            self.draw_win_or_lose(ctx)?;
        }
        graphics::present(ctx)
    }
}
