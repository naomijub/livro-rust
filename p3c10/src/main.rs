use std::thread;
use std::time::Duration;

fn main() {
//   ola();
//   join_handle();
//   join_handle_com_valor();
    // dez_mil();
    // cpus();
    // pool();
    // rayon_pool();
    
}

fn ola() {
    thread::spawn(move || {
        println!("Ola vindo da thread filha");
      });
      thread::sleep(Duration::new(1, 100));
      println!("Ola vindo da thread mãe");
}

fn join_handle() {
    let handle = thread::spawn(move || {
      println!("Ola vindo da thread filha");
    });
  
    println!("Ola vindo da thread mãe");
    let output = handle.join().unwrap();
    println!("A thread filha retornou {:?}", output);
}

fn join_handle_com_valor() {
    let handle = thread::spawn(move || {
      println!("Ola vindo da thread filha");
      42usize
    });
  
    println!("Ola vindo da thread mãe");
    let output = handle.join().unwrap();
    println!("A thread filha retornou {:?}", output);
}

static DEZ_MIL: i32 = 10_000;

fn dez_mil() {
  for i in 0..DEZ_MIL {
    let _ = thread::spawn(move || {
      println!("Chamando a thread de número {:?}", i);
    });
  }
}

fn cpus() {
    let ncpus = num_cpus::get();
    for i in 0..ncpus {
      let _ = thread::spawn(move || {
        println!("Chamando a thread de número {:?}", i);
      });
    }
    println!("Total de núcleos {:?}", ncpus);

    let ncpus_fisicas = num_cpus::get_physical();
    for i in 0..ncpus_fisicas {
        let _ = thread::spawn(move || {
          println!("Chamando a thread física de número {:?}", i);
        });
      }
      println!("Total de núcleos {:?}", ncpus_fisicas);
}

use threadpool::ThreadPool;

fn pool() {
  let ncpus = num_cpus::get_physical();
  let pool = ThreadPool::new(ncpus);

  for i in 0..ncpus {
    pool.execute(move || {
      println!("Chamando a thread de número {:?}", i);
    })
  }
  thread::sleep(Duration::new(0, 1000));
}

fn rayon_pool() {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    let n = pool.install(|| fib(20));
    println!("{}", n);
}

fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }
    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2)); // executa dentro do `pool`
    return a + b;
}