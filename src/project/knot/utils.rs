use std::sync::{Arc, RwLock};
use std::thread;


pub fn compare_ctx(ctx1: Arc<Vec<u8>>, ctx2: Arc<Vec<u8>>)
    -> Vec<(usize, Vec<u8>)>
{

    let next = Arc::new(RwLock::new(1_usize));
    let next_arc_1 = Arc::clone(&next);
    let next_arc_2 = Arc::clone(&next);

    let row_1 = Arc::new(RwLock::new(0_usize));
    let row_2 = Arc::new(RwLock::new(0_usize));
    let row_1_arc = Arc::clone(&row_1);
    let row_2_arc = Arc::clone(&row_2);

    let l1: Arc<RwLock<Vec<u8>>> = Arc::new(RwLock::new(Vec::new()));
    let l1_arc = Arc::clone(&l1);

    let l2: Arc<RwLock<Vec<u8>>> = Arc::new(RwLock::new(Vec::new()));
    let l2_arc = Arc::clone(&l2);

    let read_ctx = |
        ctx: Arc<Vec<u8>>, line: Arc<RwLock<Vec<u8>>>, next: Arc<RwLock<usize>>,
        row: Arc<RwLock<usize>>
    | {
        let mut _row = 1_usize;
        for c in ctx.iter() {
            if *c == '\n' as u8 {
                *row.write().unwrap() = _row;
                while *next.read().unwrap() == _row {}
                *line.write().unwrap() = Vec::new();
                _row += 1;
            }
            line.write().unwrap().push(*c);
        }
    };

    let read_ctx1 = thread::spawn(move || { read_ctx(ctx1, l1_arc, next_arc_1, row_1_arc) });
    let read_ctx2 = thread::spawn(move || { read_ctx(ctx2, l2_arc, next_arc_2, row_2_arc) });

    let mut diff: Vec<(usize, Vec<u8>)> = Vec::new();

    while !read_ctx1.is_finished() || !read_ctx2.is_finished() {
        if
            (*row_1.read().unwrap() == *next.write().unwrap()) &&
            (*row_2.read().unwrap() == *next.write().unwrap())
        {
            if *l1.read().unwrap() != *l2.read().unwrap() {
                diff.push((*next.read().unwrap(), l2.read().unwrap().clone()));
            }
            *next.write().unwrap() += 1;
        }
    }

    read_ctx1.join().unwrap();
    read_ctx2.join().unwrap();

    diff
}

