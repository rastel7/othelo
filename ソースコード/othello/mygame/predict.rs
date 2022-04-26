use crate::mygame::{board::Board, Stone, *};
use rand::Rng;
pub mod montecarlo;

/*
    与えられた石のターンから，お互いランダムに行動を繰り返していく
*/
fn randommove(board: &mut Board, stonecolor: Stone) {
    let mut color = stonecolor;
    let mut cnt = 0;
    while board.black_num + board.white_num < (BOARDSIZE * BOARDSIZE) as u32 {
        let list = can_set_pos(board, color);
        if list.len() != 0 {
            let _ = board.setstone(
                &can_cnt(
                    &board.gamebord,
                    color,
                    list[rand::thread_rng().gen_range(0, list.len())],
                    false,
                ),
                color,
            );
            board.count_stone();
            if board.black_num == 0 || board.white_num == 0 {
                return;
            }
            cnt = 0;
        }
        cnt += 1;
        if cnt > 1 {
            /*
                両プレイヤーが石を置けなかった場合にゲーム終了
            */
            return;
        }
        color.reversestone();
    }
}

/*
    与えられた盤面と石の色から，石をおける箇所を返す
*/
pub fn can_set_pos(board: &Board, color: crate::mygame::Stone) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = Vec::new();
    for i in 0..crate::mygame::BOARDSIZE {
        for j in 0..crate::mygame::BOARDSIZE {
            match board.gamebord[i][j] {
                Stone::Blank => {
                    //置けるか否かを判定するだけなので，can_cntはひっくり返る石が一個見つかった時点で終了させる
                    let list = can_cnt(&board.gamebord, color, (i, j), true);
                    if list.len() != 0 {
                        ret.push((i, j));
                    }
                }
                _ => {}
            }
        }
    }
    ret
}
//石を置いたときにひっくり返す石のリストを返す
pub fn can_cnt(
    board: &[[Stone; crate::mygame::BOARDSIZE]; crate::mygame::BOARDSIZE],
    color: crate::mygame::Stone,
    pos: (usize, usize),
    earlyreturn: bool, //can_set_posで使用，ひっくり返る石が一個見つかった時点で終了
) -> Vec<(usize, usize)> {
    if board[pos.0][pos.1] as i32 != Stone::Blank as i32 {
        return Vec::new();
    }
    let mut ret: Vec<(usize, usize)> = vec![(pos.0, pos.1)]; //置く位置を返り値変数に入力
    let mut opp = color;
    opp.reversestone();
    //8方向への探索を行う
    for dy in (-1)..2 {
        for dx in (-1)..2 {
            if dy == 0 && dx == 0 {
                continue;
            }
            for i in 1..BOARDSIZE as i32 {
                let (ny, nx) = (pos.0 as i32 + dy * i, pos.1 as i32 + dx * i);
                if nx < 0 || BOARDSIZE as i32 <= nx || ny < 0 || BOARDSIZE as i32 <= ny {
                    //盤面外に行ったら終了
                    break;
                }
                if board[ny as usize][nx as usize] as i32 == color as i32 && 2 <= i {
                    //同じ色の石を見つけた場合，そこから石を置いた箇所までの石がひっくり返る
                    for j in 1..i {
                        ret.push((
                            (pos.0 as i32 + dy * j) as usize,
                            (pos.1 as i32 + dx * j) as usize,
                        ));
                        if earlyreturn {
                            return ret;
                        }
                    }
                    break;
                }
                if board[ny as usize][nx as usize] as i32 != opp as i32 {
                    //石がない場所が見つかったら，この方向での探索は終了
                    break;
                }
            }
        }
    }
    //返り値の長さが1だけだった場合は，ひっくり返る石が存在しない
    if ret.len() == 1 {
        return vec![];
    }
    ret
}
