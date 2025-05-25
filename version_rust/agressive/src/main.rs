use std::io::{self,BufRead};
fn main() {
    let entrada=io::stdin();
    let mut lineas= entrada.lock().lines(); //como si fuera un readlines de py
    let tests:usize=lineas.next().unwrap().unwrap().parse().unwrap(); //leo el primer elemento (cant de tests)
    for _ in 0..tests{
        let n=lineas.next().unwrap().unwrap();
        let mut t=n.trim().split_whitespace();
        let elems:usize= t.next().unwrap().parse().unwrap();
        let cows:usize=t.next().unwrap().parse().unwrap();
        let mut stalls:Vec<usize>=Vec::with_capacity(elems);
        for _ in 0..elems{
            stalls.push(lineas.next().unwrap().unwrap().parse().unwrap());
        }
        stalls.sort(); //para poder tener desde la posicion mas cercana hasta la ultima 
        let mut d_l:usize=1; //distancia minima
        let mut d_h:usize= stalls[stalls.len()-1]-stalls[0]; //distancia maxima que puede alcanzar
        while d_l <= d_h{
            let mid:usize= (d_l+d_h)/2; //calculo el medio
            //sabiendo que a partir de una distancia invalida todas las q le siguen tmb lo son: 
            if d_valida(&stalls, cows, mid){  //veo si la distancia del medio es valida entonces sigo para la derecha 
                d_l=mid+1; 
            }else { //si esa distancia invalida todas las que le siguen tmb lo son entonces veo las distancias antes de esa
                d_h=mid-1;
            }
        }
        println!("{}",d_h);
        
    }
}


fn d_valida(stalls:&Vec<usize>,cows:usize,d:usize)->bool{
    let mut ult:usize=stalls[0]; //asigno la primer vaca a la primera posicion
    let mut cant_cows:usize=1; //le sumo 1 a la cantidad de vacas que puse
    for i in 0..stalls.len(){
        if stalls[i]-ult >= d { //si la distancia entre la ultima cow y el stall donde estoy viendo si poner a otra es >= a la distancia minima
            ult=stalls[i]; //asigno la vaca a esa posicion
            cant_cows+=1; 
        }
    }
    return cant_cows>=cows; //retorno si se pudieron asignar todas las vacas 

}