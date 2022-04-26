use crate::mygame::{Board, Stone, *};
use rand::Rng;
use std::thread;
use std::time::Instant;

/*
    CPUの思考にはモンテカルロ木探索を使用
*/
// ゲーム木のノード
#[derive(Clone)]
struct Node {
    id: usize,
    board: Board,
    parent: Option<usize>,
    color: Stone,
    childrens: Vec<usize>,
    moves: Option<(usize, usize)>,
    win: f32,
    visit: f32,
    unusedmoves: Vec<(usize, usize)>,
}
impl Node {
    pub fn new(id: usize, mov: (usize, usize), board: &Board, parent: usize, color: Stone) -> Self {
        Node {
            id: id,
            board: *board,
            parent: Some(parent),
            color: color,
            childrens: Vec::new(),
            moves: Some(mov),
            win: 0.0,
            visit: 0.0,
            unusedmoves: super::can_set_pos(board, color),
        }
    }
    /*
        子のノードの中で下記の数式が最大となるノードを選ぶ　数式はモンテカルロ木探索AIで最も慣例的に使われている物を使用
        win/visit+π*sqrt(ln(N)/visit) (Nは親ノードの試行回数)

    */
    fn select_child(&self, trees: &Vec<Node>) -> Option<usize> {
        if self.childrens.len() == 0 {
            return None;
        }
        let (mut ret_id, mut mx_score) = (0, -1e9);
        for id in self.childrens.iter() {
            let pi = std::f32::consts::PI;
            let node = &trees[*id];
            let score = (node.win as f32) / (node.visit as f32)
                + pi * ((self.visit as f32).ln() / (node.visit as f32)).sqrt();
            if score > mx_score {
                ret_id = *id;
                mx_score = score;
            }
        }
        Some(ret_id)
    }
    /*
        まだ行動可能な場合，自身の子でゲーム木が存在しない物をランダムに選択し，ゲーム木を作成する
    */
    fn expand_child(&self, trees: &mut Vec<Node>) -> Option<(Node, usize)> {
        if self.unusedmoves.len() == 0 {
            return None;
        }

        let random_id = rand::thread_rng().gen_range(0, self.unusedmoves.len());
        let mut board = self.board.clone();
        let color = self.color.return_reverse_color();
        board.setstone(
            &super::can_cnt(&board.gamebord, color, self.unusedmoves[random_id], false),
            color,
        );
        let tree = Node::new(
            trees.len(),
            self.unusedmoves[random_id],
            &board,
            self.id,
            color,
        );
        trees.push(tree.clone());
        Some((tree, random_id))
    }
    /*
        expand_childで作成したゲーム木への行動を削除する
    */
    fn deleteunusedmoves(&mut self, id: usize) {
        self.unusedmoves.remove(id);
    }
    /*
        勝敗が決定するまでお互いランダムに打つ
    */
    fn simlate(&self, color: Stone) -> f32 {
        let mut board = self.board;
        super::randommove(&mut board, self.color.return_reverse_color());

        match board.return_win() {
            Stone::Black => match color {
                Stone::Black => 1.0,
                _ => 0.0,
            },
            Stone::White => match color {
                Stone::White => 1.0,
                _ => 0.0,
            },
            Stone::Blank => 0.5,
        }
    }
}
//一回あたりの探索上限回数
/*
    staticな変数として，現在行動探索を行っているスレッドが存在するか，
    探索が終わっている場合の探索結果を持つpairの2つを使用している
*/
static mut POSITION: Option<Option<(usize, usize)>> = None;
static mut PREDICT_END: bool = true;
const MAXTRY: usize = 4000;
pub fn montecarlotree(
    board2: &Board,
    color: Stone,
    status: &crate::mygame::detailedstatus::Status,
) -> Option<Option<(usize, usize)>> {
    /*
        None:現在思考中
        Some(None):おける場所なし
        Some(Some(T)):Tへと置く
    */
    unsafe {
        //static変数の書き換えと参照はRustではunsafeとなる
        if PREDICT_END == false {
            return None; //思考中
        }
        match POSITION {
            None => {}
            _ => {
                let ret = POSITION.clone();
                if !status.rotating {
                    //値をリセットするのは，実際に行動できる，回転中の石がある場合のみ
                    POSITION = None;
                }
                return ret;
            }
        }
        PREDICT_END = false;
    }
    let board = board2.clone();
    thread::spawn(move || {
        //
        let start = Instant::now(); //予測にかかる時間の計測
        let mov = super::can_set_pos(&board, color);
        if mov.len() == 0 {
            unsafe {
                POSITION = Some(None);
                PREDICT_END = true;
            }
            return; //置ける場所なし
        }
        //現在のゲーム木を作成
        let root = Node {
            id: 0,
            board: board,
            childrens: Vec::<usize>::with_capacity(64),
            color: color.return_reverse_color(),
            moves: None,
            parent: None,
            visit: 0.0,
            win: 0.0,
            unusedmoves: mov,
        };
        let mut tree: Vec<Node> = Vec::<Node>::with_capacity(2048);
        tree.push(root);
        //最大試行回数まで探索を行う
        for _ in 0..MAXTRY {
            let mut node_id = 0;
            while tree[node_id].unusedmoves.len() == 0 && tree[node_id].childrens.len() != 0 {
                node_id = tree[node_id].select_child(&tree).unwrap();
            }
            if tree[node_id].unusedmoves.len() != 0 {
                let val = tree[node_id].clone();
                let (_node, id) = val.expand_child(&mut tree).unwrap();
                let num = tree.len().clone();
                tree[node_id].deleteunusedmoves(id);
                tree[node_id].childrens.push(num - 1);
                node_id = num - 1;
            }
            let won = tree[node_id].simlate(tree[1].color);
            let mut id_list: Vec<usize> = vec![node_id];
            while tree[node_id].parent != None {
                let id = tree[node_id].parent.unwrap();
                id_list.push(id);
                node_id = id;
            }
            for iter in id_list {
                tree[iter].win += won;
                tree[iter].visit += 1.0;
            }
        }
        //現在のゲーム木が持つ子ノードの中で，一番試行回数が大きいものを探索結果とする
        let (mut ret, mut mx_score) = ((0, 0), 0.0);
        for i in tree[0].childrens.iter() {
            if tree[*i].visit > mx_score {
                ret = tree[*i].moves.unwrap();
                mx_score = tree[*i].visit;
            }
        }
        unsafe {
            POSITION = Some(Some(ret));
            PREDICT_END = true;
        }
        let end = start.elapsed();
        println!(
            "モンテカルロ木探索予測時間 :{}.{:03}秒",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
    });
    None
}
