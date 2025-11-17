use std::sync::{Arc, RwLock};
use std::thread;


pub fn compare_ctx(ctx1: Arc<Vec<u8>>, ctx2: Arc<Vec<u8>>)
    -> (Vec<(usize, Vec<u8>)>, Vec<(usize, Vec<u8>)>)
{

    let line_num = Arc::new(RwLock::new(0_usize));
    let line_num_1 = Arc::clone(&line_num);
    let line_num_2 = Arc::clone(&line_num);

    let line_1: Arc<RwLock<Vec<u8>>> = Arc::new(RwLock::new(Vec::new()));
    let line_1_arc = Arc::clone(&line_1);
    let line_2: Arc<RwLock<Vec<u8>>> = Arc::new(RwLock::new(Vec::new()));
    let line_2_arc = Arc::clone(&line_2);

    let read_ctx = |ctx: Arc<Vec<u8>>, line: Arc<RwLock<Vec<u8>>>, num: Arc<RwLock<usize>>| {
        let mut _num = 1_usize;
        for c in ctx.iter() {
            if *c == '\n' as u8 {
                while *num.read().unwrap() <= _num {}
            }
            line.write().unwrap().push(*c);
        }
    };

    let read_ctx1 = thread::spawn(move || { read_ctx(ctx1, line_1_arc, line_num_1) });
    let read_ctx2 = thread::spawn(move || { read_ctx(ctx2, line_2_arc, line_num_2) });

    let rem: Vec<(usize, Vec<u8>)> = Vec::new();
    let add: Vec<(usize, Vec<u8>)> = Vec::new();

    while !read_ctx1.is_finished() || !read_ctx2.is_finished() {
        println!("ctx1 - {}", line_num.read().unwrap());
        println!("ctx2 - {}", line_num.read().unwrap());
        *line_num.write().unwrap() += 1;
    }

    read_ctx1.join().unwrap();
    read_ctx2.join().unwrap();

    (rem, add)
}


/*
    for c1 in ctx1 {
        if *c1 == '\n' as u8 {
            for c2 in ctx2 {
                if *c2 == '\n' as u8 {

                }
                l2.push(c1);
            }
            line += 1;
        }
        l1.push(c1);
    }
*/