use std::{thread, time};
use std::sync::{Arc, Mutex};

use getch_rs::{Getch, Key};

use block::{BlockKind, BLOCKS};

mod block;

// フィールドサイズ
const FIELD_WIDTH: usize = 11 + 2;
// フィールド＋壁
const FIELD_HEIGHT: usize = 20 + 1;

// フィールド＋底
type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];


struct Position {
    x: usize,
    y: usize,
}

// ブロックがフィールドに衝突する場合は`true`を返す
fn is_collision(field: &Field, pos: &Position, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if y + pos.y >= FIELD_HEIGHT || x + pos.x >= FIELD_WIDTH {
                continue;
            }
            if field[y + pos.y][x + pos.x] & BLOCKS[block as usize][y][x] == 1 {
                return true;
            }
        }
    }
    false
}

// フィールドを描画する
fn draw(field: &Field, pos: &Position, block: BlockKind) {
    // 描画用フィールドの生成
    let mut field_buf = field.clone();
    // 描画用フィールドにブロックの情報を書き込む
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[block as usize][y][x] == 1 {
                field_buf[y + pos.y][x + pos.x] = 1;
            }
        }
    }
    // フィールドを描画
    println!("\x1b[H");  // カーソルを先頭に移動
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field_buf[y][x] == 1 {
                print!("[]");
            } else {
                print!(" .");
            }
        }
        println!();
    }
}

fn main() {
    let field = Arc::new(Mutex::new([
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ]));

    let pos = Arc::new(Mutex::new(Position { x: 4, y: 0 }));
    let block = Arc::new(Mutex::new(rand::random::<BlockKind>()));

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");
    // フィールドを描画
    draw(&field.lock().unwrap(), &pos.lock().unwrap(), *block.lock().unwrap());

    // 自然落下処理
    {
        let pos = Arc::clone(&pos);
        let field = Arc::clone(&field);
        let block = Arc::clone(&block);
        let _ = thread::spawn(move || {
            loop {
                // 1秒間スリーブする
                thread::sleep(time::Duration::from_millis(300));
                // 自然落下
                let mut pos = pos.lock().unwrap();
                let mut field = field.lock().unwrap();
                let mut block = block.lock().unwrap();
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, *block) {
                    // posの座標を更新
                    *pos = new_pos;
                } else {
                    // ブロックをフィールドに固定
                    for y in 0..4 {
                        for x in 0..4 {
                            if BLOCKS[*block as usize][y][x] == 1 {
                                field[y + pos.y][x + pos.x] = 1;
                            }
                        }
                    }
                    // ラインの削除処理
                    for y in 1..FIELD_HEIGHT - 1 {
                        let mut can_erase = true;
                        for x in 1..FIELD_WIDTH - 1 {
                            if field[y][x] == 0 {
                                can_erase = false;
                                break;
                            }
                        }
                        if can_erase {
                            for y2 in (2..=y).rev() {
                                field[y2] = field[y2 - 1];
                            }
                        }
                    }
                    // posの座標を初期値へ
                    *pos = Position { x: 4, y: 0 };
                    *block = rand::random();
                }
                // フィールドを描画
                draw(&field, &pos, *block);
            }
        });
    }

    // キー入力処理
    let g = Getch::new();
    loop {
        // キー入力待ち
        match g.getch() {
            Ok(Key::Left) => {
                let mut pos = pos.lock().unwrap();
                let mut field = field.lock().unwrap();
                let mut block = block.lock().unwrap();
                let new_pos = Position {
                    x: pos.x - 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, *block) {
                    // posの座標を更新
                    *pos = new_pos;
                }
                draw(&field, &pos, *block);
            }
            Ok(Key::Down) => {
                let mut pos = pos.lock().unwrap();
                let mut field = field.lock().unwrap();
                let mut block = block.lock().unwrap();
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, *block) {
                    // posの座標を更新
                    *pos = new_pos;
                }
                draw(&field, &pos, *block);
            }
            Ok(Key::Right) => {
                let mut pos = pos.lock().unwrap();
                let mut field = field.lock().unwrap();
                let mut block = block.lock().unwrap();
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, *block) {
                    // posの座標を更新
                    *pos = new_pos;
                }
                draw(&field, &pos, *block);
            }
            Ok(Key::Char('q')) => break,
            _ => (),  // 何もしない
        }
    }
}
