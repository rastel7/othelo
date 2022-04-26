use ggez::Context;
use glam::*;
use mint;
pub struct MouseInf {
    pub pos: mint::Point2<f32>,
    pub pressed: bool,
}

impl MouseInf {
    /*
        フレームの最後に呼び出すこと
        現フレームでのキー入力を記録しておくことで，次フレームでキーが入力された瞬間か否かを判定する
    */
    pub fn set_mouseinf(&mut self, _ctx: &mut Context) {
        self.pos = ggez::input::mouse::position(_ctx);
        self.pressed =
            ggez::input::mouse::button_pressed(_ctx, ggez::input::mouse::MouseButton::Left);
    }
    //クリックされた瞬間ならばtrue，それ以外ならばfalse
    pub fn is_clicked(&self, _ctx: &mut Context) -> bool {
        if ggez::input::mouse::button_pressed(_ctx, ggez::input::mouse::MouseButton::Left)
            && !self.pressed
        {
            true
        } else {
            false
        }
    }
}
