use ggez::audio::{SoundSource, Source};
pub struct Se {
    stone: Source,  //石を置いた時のSE
    button: Source, //ボタンを押した時のSE
}
impl Se {
    pub fn new(ctx: &mut ggez::Context) -> Se {
        let mut stone = Source::new(ctx, "/stone.mp3").unwrap();
        let mut button = Source::new(ctx, "/button.mp3").unwrap();
        button.set_volume(0.5);//少々うるさいのでボリュームを下げる
        stone.set_volume(0.5);//少々うるさいのでボリュームを下げる
        Se {
            stone: stone,
            button: button,
        }
    }
    pub fn play_stone(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        self.stone.play_detached(ctx).unwrap();
        Ok(())
    }
    pub fn play_button(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        self.button.play_detached(ctx).unwrap();
        Ok(())
    }
}
