use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let mut vida_goku = 1000;
    let recursos = Arc::new(Mutex::new(vida_goku));

    for cell in 1..5 {
        let mutex = recursos.clone();
        thread::spawn(move || {
          let mut vida_goku = mutex.lock().expect("Goku se defendeu através do lock");
          *vida_goku -= cell * 50;
      }).join().expect("União dos golpes falhou");
    }
    thread::sleep(Duration::new(2, 0));
    vida_goku = *recursos.lock().unwrap();
    println!("A vida de Goku apos os ataque: {}", vida_goku);
}
