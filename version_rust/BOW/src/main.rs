use std::io::{self,BufRead};
use std::convert::TryInto;
use std::cmp;
use std::usize::MAX;
fn main() {
    let entrada=io::stdin();
    let mut lineas= entrada.lock().lines();
    let mut cant_elems:isize=lineas.next().unwrap().unwrap().parse().unwrap();
    while cant_elems !=-1 {
        let elems:usize=cant_elems.try_into().unwrap();
        let mut arr:Vec<usize>=Vec::with_capacity(elems+2); //inicializo con esa capacidad ya que voy a agregar al principio y al final los casos bases de black y white
        let n=lineas.next().unwrap().unwrap(); 
        let mut val=n.trim().split_whitespace();
        arr.push(0); //el caso base del black (cualquier elemento es mayor que 0 en borw ya que para todo Xi de arr, Xi>0)
        for _ in 0..elems{
            arr.push(val.next().unwrap().parse().unwrap()); 
        }
        arr.push(MAX); //el caso base del white (cualquier elem es menor al max num representable)
        let mut arr_bow:Vec<Vec<i32>>= vec![vec![-1;elems+2];elems+2]; //creo una lista de listas (ambas con mismo tamaño que el arr final) en la que cada sublista
        //está llena de -1 para indicar que ese camino todavia no fue resuelto (memorizacion)
        let res:i32=memo(&arr,&mut arr_bow,elems+1,0,1,elems); //llamo a la memorizacion teniendo en cuenta q arr[w]=MAX y arr[b]=0 y
        //ya que la lista tiene el primer elem que es el caso base, empiezo a iterar desde i=1 (primer elem de la lista original) 
        println!("{}",res);
        cant_elems=lineas.next().unwrap().unwrap().parse().unwrap();
    }
}

fn memo(arr:&Vec<usize>,arr_bow:&mut Vec<Vec<i32>>,w:usize,b:usize,i:usize,n:usize)->i32{ //memorizacion de caminos usando recursion, devuelve el minimo numero de elems sin pintar
    if i>n { //i=elems + 1 que no pertenece a la lista original (arr[w]=MAX) 
        return 0;
    } else if arr_bow[w][b]!=(-1){ //en el caso en que esté definido el camino
        return arr_bow[w][b];
    }else if arr[w]>arr[i] && arr[b]<arr[i]{ //si el elem es menor que el ultimo agregado al white y al mismo tiempo mayor al ultimo black agregado
        arr_bow[w][b]= cmp::min(1+memo(arr,arr_bow,w,b,i+1,n),cmp::min(memo(arr,arr_bow,i,b,i+1,n),memo(arr,arr_bow,w,i,i+1,n)));
        //veo cada caso, no pintarlo, pintarlo de blanco o pintarlo de negro y de ahi ver cual es la solucion que menos elementos sin pintar me deja
        return arr_bow[w][b];

    }else if arr[w]>arr[i] {
        arr_bow[w][b]= cmp::min(1+memo(arr,arr_bow,w,b,i+1,n),memo(arr,arr_bow,i,b,i+1,n));
        //al saber que no se cumple el caso de black solo veo de no pintarlo o pintarlo de blanco
        return arr_bow[w][b];
    }else if arr[b]<arr[i]{
        arr_bow[w][b] =cmp::min(1+memo(arr,arr_bow,w,b,i+1,n),memo(arr,arr_bow,w,i,i+1,n));
                //al saber que no se cumple el caso de white solo veo de no pintarlo o pintarlo de negro
        return arr_bow[w][b];
    } else {
        arr_bow[w][b]=1+memo(arr,arr_bow,w,b,i+1,n);
        //sabiendo que no se puede pintar sumo 1 a la cant de elems sin pintar y paso al siguiente elem 
        return arr_bow[w][b];
    }
}