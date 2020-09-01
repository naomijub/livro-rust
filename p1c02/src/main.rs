use std::thread::Result;
fn main() -> Result<()>{
    let thread_poder = std::thread::spawn(|| {
        println!("Medindo o ki");
        return 10000;
    });
    assert!(thread_poder.join()? >= 8000);

    Ok(())
}

// Versão antiga
// let thread_poder = std::thread::spawn(|| {
//     println!("Medindo o ki");
//     return 10000;
// });
// assert!(try!(thread_poder.join()) >= 8000);

