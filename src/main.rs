
use rand::Rng;

fn in_circle()->bool{
    let mut rnd = rand::thread_rng();
    let x = rnd.gen_range(0.0..1.0);
    let y = rnd.gen_range(0.0..1.0);
    if x*x + y*y < 1.0{
        return true;
    }

    false
}



fn main() {
    let mut count = 0;
    let run_time = 1000000;
    for _ in 0..run_time{
        if in_circle(){
            count += 1;
        }
    }

    println!("Pi is {}", 4.0 * count as f64 / run_time as f64);
}
