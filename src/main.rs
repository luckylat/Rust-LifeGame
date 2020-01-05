use std::{thread, time};
use std::time::Duration;

mod life;

fn main() {
    let sz : usize = 10;
    let mut stge = life::new(sz);

    stge[2][0] = true;
    stge[2][1] = true;
    stge[2][2] = true;
    stge[1][2] = true;
    stge[0][1] = true;


    loop {
        print!("\x1B[2J");
        for x in &stge {
            let mut text = String::new();
            for y in x{
                text = format!("{}{}", text, if y == &true {"#"} else {"."});
            }
            println!("{}",text);
        }
        let nxt = life::life(sz,&stge);
        stge = nxt;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
}
