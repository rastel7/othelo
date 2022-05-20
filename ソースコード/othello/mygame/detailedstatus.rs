use ggez::GameResult;
pub struct Status {
    think_description: f32,//文字を点滅させるのに使用
    placed_rotating_stone_count: f32,//ひっくり返る候補の石を表示するのに使用
    pub rotating: bool,//回転中の石があるか否か
    pub thinking: bool,//CPUが思考中
    pub game_end: bool,//ゲーム終了時のパーティクル発生に使用
}
impl Status {
    pub fn new(blink_description: f32) -> Self {
        Status {
            think_description: blink_description,
            placed_rotating_stone_count: 0.0,
            rotating: false,
            thinking: false,
            game_end: false,
        }
    }
    pub fn update_status(&mut self, elasped_time: f32) -> GameResult {
        self.think_description += elasped_time;
        self.think_description %= 1.0;
        self.placed_rotating_stone_count -= elasped_time * 60.0;
        self.placed_rotating_stone_count = (self.placed_rotating_stone_count + 100.0) % 100.0;
        Ok(())
    }
    pub fn get_blink_description(&self) -> f32 {
        self.think_description
    }
    pub fn get_placed_rotating_stone_count(&self) -> f32 {
        self.placed_rotating_stone_count
    }
}
