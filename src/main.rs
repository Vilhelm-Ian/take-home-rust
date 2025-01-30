use std::{thread, time};

fn main() {
    let mut count = 0;
    ticker(move || {
        println!("{count}");
        count += 1;
    });
    println!("Hello, world!");
}

fn ticker<F>(mut f: F)
where
    F: FnMut(),
{
    loop {
        f();
        thread::sleep(time::Duration::from_secs(1));
    }
}
