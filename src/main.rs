use std::thread;
use std::cmp;
use std::sync::mpsc;
use std::sync::{Mutex,Arc};
fn main() {
    println!("Hello concurrent world!");


    
    let input_guarded = Arc::new(Mutex::new(vec![1u32,4,3,500,10,20,6,3,2,0,2]));

    let sum = || -> u32 { let mut sum : u32 = 0;
                    let input = input_guarded.lock().unwrap();
                    for v in input.iter() {
                       sum += v; 
                }
        sum
    };

    println!("Sequential sum of input vector is {}", sum() );

    let (tx, rx) = mpsc::channel();

    let threads = [0,1];
    
    for i in 0..threads.len(){
        let txc = tx.clone();
        let input_guarded = Arc::clone(&input_guarded);
        thread::spawn(move || {
            let mut sum : u32 = 0;
            let input = input_guarded.lock().unwrap();
            let chunk_size = (input.len()+ 1)/threads.len();
            let offset = chunk_size*i; 
            let border = cmp::min(offset+chunk_size,input.len());
            for o in offset..border {
                sum+=input[o];
            }
            txc.send(sum).unwrap();
        }); 
    }

    let mut total_sum = 0; 
    for i in 0..threads.len() {
        total_sum += rx.recv().unwrap();
    }
    println!("Concurrent sum of input vector is {}", total_sum);
}
