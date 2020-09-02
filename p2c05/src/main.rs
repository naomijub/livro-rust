fn main() {
    fn inc(x: i32) -> i32 {
        x + 1
    }
    
    let f = inc;
    let mut dois = f(2);

    inc_mut(&mut dois);
    println!("{}", dois);
}

fn inc(x: i32) -> i32 {
    x + 1
}

fn inc_mut(x: &mut i32) {
    *x += 1;
}

fn ordem_superior<F>(valor:i32, func: F)  -> i32
                    where F: Fn(i32) -> i32 {
    func(valor)
}

#[test]
fn fun_ordem_superior() {
    let onze = 11i32;
  assert_eq!(onze, ordem_superior(10i32, inc))
}

#[test]
fn closure() {
    let inc = |x: i32| x + 1;

    assert_eq!(2, inc(1));
}

#[test]
fn closure_mut() {
    let mut val = 1i32;
    let inc = |x: &mut i32| *x += 1;
    inc(&mut val);

    assert_eq!(2, val);
}

