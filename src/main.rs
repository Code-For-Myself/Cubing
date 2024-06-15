use ndarray::arr2;
use std::io;

type K = ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>>;
fn main() {
    let mut a: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>> = arr2( 
        &[
            [0, 0, 1, 1, 0, 0, 0, 0],
            [0, 0, 1, 1, 0, 0, 0, 0,],
            [3, 3, 2, 2, 5, 5, 6, 6,],
            [3, 3, 2, 2, 5, 5, 6, 6,],
            [0, 0, 4, 4, 0, 0, 0, 0,],
            [0, 0, 4, 4, 0, 0, 0, 0,],
            [0, 0, 6, 6, 0, 0, 0, 0,],
            [0, 0, 6, 6, 0, 0, 0, 0,]
            ]);
    
    let mut counter = 0;
    let mut number = String::new();
    println!("{}",a);
    println!("How many times u want to input something and get a nice cube-result in the perfect ugly matrix-with-numbers view?");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let number: i32 = number.trim().parse().unwrap();
    loop {
        let mut guess = String::new();
        counter+=1;
        println!("Please input r1, r2, u1 or u2");
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        let anew = read_match_out(&mut guess, a.clone());
        a = anew;
        if counter == number {
            break;
        }
        /* println!("{}", a);
        println!("=");
        println!("{}",up2(a)); */
    }
}
fn read_match_out(guess: &str, a: K)-> K{
        match guess.trim() {
            "u1" => {println!("{}",u1(a.clone()));
                    u1(a)
        },
            "u2" => {println!("{}",u2(a.clone()));
                    u2(a)
        },
            "r1" => {println!("{}",r1(a.clone()));
                    r1(a)
        },
            "r2" => {println!("{}",r2(a.clone()));
                    r2(a)
        },   _ => {println!("cant read input");
            a
        }
        
    }
}
fn r1(mut a: K) ->K{
    let mut vec: Vec<i32> = Vec::new();
    for i in 0..8{
        vec.push(a[[2, (i+6)%8]])
    }
    for i in 0..8 {
        a[[2,i]] = vec[i]
    }
    (a[[7,3]],a[[7,2]]) = (a[[2,6]], a[[2,7]]);
    (a[[0,2]],a[[0,3]],a[[1,3]],a[[1,2]]) = (a[[0,3]],a[[1,3]],a[[1,2]],a[[0,2]]);

    a
}

fn r2(mut a: K) -> K{
    let mut vec: Vec<i32> = Vec::new();
    for i in 0..8{
        vec.push(a[[3, (i+6)%8]])
    }
    for i in 0..8 {
        a[[3,i]] = vec[i]
    }
    (a[[6,3]],a[[6,2]]) = (a[[3,6]], a[[3,7]]);
    (a[[4,3]],a[[5,3]],a[[5,2]],a[[4,2]])=(a[[4,2]],a[[4,3]],a[[5,3]],a[[5,2]]);
    a
}
fn u1(mut a: K) -> K {
    let mut vec: Vec<i32> = Vec::new();
    for i in 0..8{
        vec.push(a[[(i+6)%8, 2]])
    }
    for i in 0..8 {
        a[[i, 2]] = vec[i]
    }
    (a[[2,7]],a[[3,7]]) = (a[[7,2]], a[[6,2]]);
    (a[[2,0]],a[[2,1]],a[[3,1]],a[[3,0]]) = (a[[2,1]],a[[3,1]],a[[3,0]],a[[2,0]]);
    a
}
fn u2(mut a: K) -> K {
    let mut vec: Vec<i32> = Vec::new();
    for i in 0..8{
        vec.push(a[[(i+6)%8, 3]])
    }
    for i in 0..8 {
        a[[i, 3]] = vec[i]
    }
    (a[[2,6]],a[[3,6]]) = (a[[7,3]], a[[6,3]]);
    (a[[2,5]],a[[3,5]],a[[3,4]],a[[2,4]])=(a[[2,4]],a[[2,5]],a[[3,5]],a[[3,4]]);
    a
}

