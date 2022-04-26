use crate::mygame::{board::Board, mouse, stone::Stone, ButtonEventList};
use ggez::{graphics, Context, GameResult};
use glam::*;
use mint;
const BUTTON_ROUND: f32 = 10.0;
/*
    UIとしてクリックするボタンを管理するTrait
*/
pub struct UIs {
    pub buttons: Vec<Box<dyn Button>>,
}
impl UIs {
    /*
        パスボタンとリセットボタンを宣言し，Box化して配列として持つ
    */
    pub fn new() -> UIs {
        UIs {
            buttons: vec![
                Box::new(Reset::new(
                    mint::Point2 {
                        x: crate::WIDTH / 2.0 - 100.0,
                        y: crate::HEIGHT - 40.0,
                    },
                    mint::Point2 {
                        x: crate::WIDTH / 2.0 - 5.0,
                        y: crate::HEIGHT,
                    },
                    "reset",
                )),
                Box::new(Pass::new(
                    mint::Point2 {
                        x: crate::WIDTH / 2.0 + 5.0,
                        y: crate::HEIGHT - 40.0,
                    },
                    mint::Point2 {
                        x: crate::WIDTH / 2.0 + 100.0,
                        y: crate::HEIGHT,
                    },
                    "pass",
                )),
            ],
        }
    }
}
pub trait Button {
    /*
        ボタンの左上(left_up)と右下(right_bottom)を定義する
    */
    fn poslu(&self) -> &mint::Point2<f32>;
    fn posrb(&self) -> &mint::Point2<f32>;
    fn clicked(&self, left_click: &mouse::MouseInf, ctx: &mut Context) -> Option<bool> {
        /*
            クリックされてたらSome(true)
            マウスオーバーのみはSome(false)
            それ以外はNone
        */
        let over = self.poslu().x <= left_click.pos.x
            && self.poslu().y <= left_click.pos.y
            && self.posrb().x >= left_click.pos.x
            && self.posrb().y >= left_click.pos.y;
        if !left_click.is_clicked(ctx) && !over {
            return None;
        }
        if left_click.is_clicked(ctx) && over {
            return Some(true);
        }
        if !left_click.is_clicked(ctx) && over {
            return Some(false);
        }
        return None;
    }
    /*
    描画用関数
    マウスオーバーした時に変化を持たせるため，マウス情報も受け取る
    */
    fn draw(
        &self,
        ctx: &mut Context,
        font: &graphics::Font,
        mouse: &mouse::MouseInf,
        board: &Board,
    ) -> GameResult<()>;
    /*
        ボタンが押されたときの行動を定義
        mutなmy_game構造体等のゲームに対しての影響が大きいデータを受け取りたくないため，
        イベントの種類のみを返して，特別な処理は行わない
    */
    fn action(&self) -> GameResult<ButtonEventList>;
}
struct Reset {
    poslu: mint::Point2<f32>,
    posrb: mint::Point2<f32>,
    text: String, //ボタンに表示する文字列
}
impl Reset {
    pub fn new(poslu: mint::Point2<f32>, posrb: mint::Point2<f32>, text: &str) -> Reset {
        Reset {
            poslu: poslu,
            posrb: posrb,
            text: text.to_string(),
        }
    }
}
impl Button for Reset {
    fn poslu(&self) -> &mint::Point2<f32> {
        &self.poslu
    }
    fn posrb(&self) -> &mint::Point2<f32> {
        &self.posrb
    }
    fn draw(
        &self,
        ctx: &mut Context,
        font: &graphics::Font,
        mouse: &mouse::MouseInf,
        _board: &Board,
    ) -> GameResult<()> {
        let rect_siz = graphics::Rect {
            x: self.poslu().x,
            y: self.poslu().y,
            w: (self.posrb().x - self.poslu().x),
            h: (self.posrb().y - self.poslu().y),
        };
        //マウスオーバー中のみ色を変える
        let color = match self.clicked(mouse, ctx) {
            Some(false) => graphics::Color::new(0.7, 0.7, 0.7, 0.7),
            _ => graphics::Color::new(1.0, 1.0, 1.0, 0.7),
        };
        let rect = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect_siz,
            BUTTON_ROUND,
            graphics::Color::WHITE,
        )
        .unwrap();
        let text: String = self.text.to_string();
        let text = graphics::Text::new((text, *font, 32.0));
        //まずは下側に表示される四角形から描画
        graphics::draw(ctx, &rect, ((Vec2::new(0.0, 0.0)), color))?;
        //ボタンテキストを描画
        graphics::draw(
            ctx,
            &text,
            (
                (Vec2::new(
                    rect_siz.center().x - text.width(ctx) / 2.0,
                    rect_siz.center().y - text.height(ctx) / 1.9,
                )),
                graphics::Color::BLACK,
            ),
        )?;
        Ok(())
    }
    fn action(&self) -> GameResult<ButtonEventList> {
        //実行してほしいイベントを返す
        Ok(ButtonEventList::Reset)
    }
}

pub struct Pass {
    poslu: mint::Point2<f32>,
    posrb: mint::Point2<f32>,
    text: String,
}
impl Pass {
    pub fn new(poslu: mint::Point2<f32>, posrb: mint::Point2<f32>, text: &str) -> Pass {
        Pass {
            poslu: poslu,
            posrb: posrb,
            text: text.to_string(),
        }
    }
}
impl Button for Pass {
    fn poslu(&self) -> &mint::Point2<f32> {
        &self.poslu
    }
    fn posrb(&self) -> &mint::Point2<f32> {
        &self.posrb
    }
    fn draw(
        &self,
        ctx: &mut Context,
        font: &graphics::Font,
        mouse: &mouse::MouseInf,
        board: &Board,
    ) -> GameResult<()> {
        if super::predict::can_set_pos(&board, Stone::White).len() != 0 {
            return Ok(());
        }
        let rect_siz = graphics::Rect {
            x: self.poslu().x,
            y: self.poslu().y,
            w: (self.posrb().x - self.poslu().x),
            h: (self.posrb().y - self.poslu().y),
        };
        let color = match self.clicked(mouse, ctx) {
            Some(false) => graphics::Color::new(0.7, 0.7, 0.7, 0.7),
            _ => graphics::Color::new(1.0, 1.0, 1.0, 0.7),
        };
        let rect = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect_siz,
            BUTTON_ROUND,
            graphics::Color::WHITE,
        )
        .unwrap();
        let text: String = self.text.to_string();
        let text = graphics::Text::new((text, *font, 32.0));
        let _ = graphics::draw(ctx, &rect, ((Vec2::new(0.0, 0.0)), color));
        let _ = graphics::draw(
            ctx,
            &text,
            (
                (Vec2::new(
                    rect_siz.center().x - text.width(ctx) / 2.0,
                    rect_siz.center().y - text.height(ctx) / 1.9,
                )),
                graphics::Color::new(0.0, 0.0, 0.0, 1.0),
            ),
        );
        Ok(())
    }
    fn action(&self) -> GameResult<ButtonEventList> {
        Ok(ButtonEventList::Pass)
    }
}
