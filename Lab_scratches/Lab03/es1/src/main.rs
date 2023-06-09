mod my_lib_mod;
mod my_thread_mod;
use crate::my_lib_mod::lib::*;
use crate::my_thread_mod::my_thread::*;


use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;




fn main() {
    let cifre = [1,2,3,4,5,6,7,8].to_vec();
    let elements = vec!['+', '-', '/', '*'];
    let mut sol : Vec<(Vec<i32>,Vec<Vec<char>>)> = Vec::new();

    //sembra però che la divisione in thread addirittura aumenti i tempi di esecuzione
    let n = 5;
    

    //la funzione calcola le permutazioni senza ripetizioni (se due permutazioni risultano uguali allora le cancella)
    let perm = permutazioni(&cifre);
    let chunk_size = (perm.len() + n - 1) / n;  
    

    //ora devo fare in modo di usare +,-,/ e * in tutte le combinazioni possibili
    let dispositions = dispositions_with_repetition(&elements, 4);


    //questa funzione trova tutte le possibili soluzioni per arrivare a 10 dato un vettore di permutazioni ed un vettore di permutazioni
    let start = Instant::now();

    find_sol(&mut sol,&perm,&dispositions);
    let string_sol = sol_into_string(&sol);
    let end = Instant::now();
   

    for x in &string_sol{
        println!("{:?}",x);
    }
    

    //divido le permutazioni (dunque il Vec<Vec<i32>>) in un vettore di vettori di permutazioni (cioè in un vettore Vec di chunck, percio Vec<Vec> di permutazioni e quindi Vec<Vec<Vec>>)
    let handles : Vec<Vec<Vec<i32>>>=  perm.chunks(chunk_size).map(|c| c.to_vec()).collect() ;
    

    //qui sotto comincia l'implementazione con i thread
    let shared_sol_as_string = Arc::new(Mutex::new(Vec::new()));
    let shared_sol = Arc::new(Mutex::new(sol));
    let shared_disp: Arc<Mutex<Vec<Vec<char>>>> = Arc::new(Mutex::new(dispositions.clone()));

    let mut threads = vec![];

    let now_t = Instant::now();

    for i in 1..n{
        let shared_handles = Arc::new(Mutex::new(handles[i].clone()));
        
        let arc_sol: Arc<Mutex<Vec<(Vec<i32>, Vec<Vec<char>>)>>> = shared_sol.clone(); 
        let arc_sol_2: Arc<Mutex<Vec<(Vec<i32>, Vec<Vec<char>>)>>> = shared_sol.clone(); 
        let arc_hand: Arc<Mutex<Vec<Vec<i32>>>> = shared_handles.clone();
        let arc_disp: Arc<Mutex<Vec<Vec<char>>>> = shared_disp.clone();

        let arc_sol_as_string = shared_sol_as_string.clone();

        threads.push(thread::spawn(move ||{
            thread_find_sol(arc_sol, arc_hand, arc_disp);
            thread_sol_into_string(arc_sol_2,arc_sol_as_string);
        }));
    }
  
    
    for t in threads { t.join().unwrap(); }


    let end_t = Instant::now();

    let duration_t = end_t.duration_since(now_t).as_micros();

    let duration = end.duration_since(start);
    

    for x in &(*shared_sol_as_string.lock().unwrap()){
        println!("{:?}",x);
    }

    println!("Durata programma NO THREAD in millisecondi: {:?}",((duration.as_micros() as f32)/1000.0) );
    println!("Durata programma CON THREAD in millisecondi: {:?}",((duration_t as f32)/1000.0) );
    
}
