use std::io::{self,BufRead};
fn main() {
    let entrada=io::stdin();
    let mut lineas= entrada.lock().lines(); //como si fuera un readlines de py
    let tests:usize=lineas.next().unwrap().unwrap().parse().unwrap(); //leo el primer elemento (cant de tests)
    for i in 0..tests{
        let acts:usize=lineas.next().unwrap().unwrap().parse().unwrap(); //cant de actividades que va a tener el test
        let mut hrs:Vec<(usize,usize)>=Vec::with_capacity(acts); //creo la variable fuera para poder utilizarla luego en la funcion
        for a in 0..acts{
            let n=lineas.next().unwrap().unwrap(); //me sacaba error si juntaba n e in_fin en una sola variable :(
            let mut in_fin=n.trim().split_whitespace();
            hrs.push((in_fin.next().unwrap().parse().unwrap(),in_fin.next().unwrap().parse().unwrap())); 
        }
        hrs.sort_by(|a,b| a.1.cmp(&b.1)); //sorteo el vector por el elemento 1 de las tuplas para q tenga ordenado desde la menor hora de salida
        println!("{}",greedy(&hrs));
    }
}

fn greedy(n:&[(usize,usize)])->usize{  //uso fonraGreedy para devolver la cant max de actividades que se pueden hacer en 12 hrs
    let mut last:usize=0;
    let mut total:usize=0;
    for i in (0..n.len()) {
        if n[i].0>=last{ //
            last= n[i].1;
            total+=1;
        }
    }
    return total 
    
}