#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate clap;
use clap::{App, Arg};

mod instrumented_algorithms;
mod lib;
// mod parse;
mod rttrace;



// use instrumented_algorithms::init_arr;
// use crate::rttrace::Data;
use instrumented_algorithms::{init,multiply,multiply_s, init_i32};
// use nanorand::{Rng, WyRand};
use lib::{reuse_distance_eff};

// use crate::rttrace::Data;
// use crate::rttrace::{init,trace};

use std::env;
use std::fs::File;
use std::io::prelude::*;



fn main() -> std::io::Result<()> {
    test_mm_element()?;
    
    
    //test_stras()?;
    // let mut a = Vec::new();
    // let mut b = Vec::new();
    // a.push(vec![1,2]);
    // a.push(vec![1,2]);
    // b.push(vec![1,2]);
    // b.push(vec![1,2]);

    // let (c, mmdata) = strassen(&mut a,&mut b);
    // println!("{:?}", c);
    Ok(())
}

fn test_mm_element() -> std::io::Result<()> {
    let matches = App::new("Matrix multiplication")
    .author("Aidan Goldfarb <agoldfa7@u.rochester.edu>")
    .about("Matrix multiplication tracking rdd of only a single specified element")
    .arg(
        Arg::with_name("SIZE")
            .short("s")
            .long("size")
            .help("Sets matrix dimensions")
            .required(true)
            .takes_value(true),
    )
    .arg(
        Arg::with_name("X_VALUE")
            .short("x")
            .long("x")
            .help("specifies x or row to track")
            .takes_value(true)
            .default_value("0")
            .required(true),
    )
    .arg(
        Arg::with_name("Y_VALUE")
            .short("y")
            .long("y")
            .help("specifies y or column to track")
            .takes_value(true)
            .default_value("0")
            .required(true),
    )
    .arg(
        Arg::with_name("matrix")
            .short("mat")
            .long("matrix")
            .help("specifies whether to track val in A or B")
            .takes_value(true)
            .required(true),
    )
    .get_matches();

    let size: usize = matches.value_of("SIZE").unwrap().parse::<usize>().unwrap();

    let x: usize = matches.value_of("X_VALUE").unwrap().parse::<usize>().unwrap();
    let y: usize = matches.value_of("Y_VALUE").unwrap().parse::<usize>().unwrap();
    let mat = matches.value_of("matrix").unwrap();
    let (mut a, mut b) = init(size);
    let val; 
    //let do_a;
    
    if mat.eq("a") || mat.eq("A"){
        //do_a = true;
        val = a[x][y];
    }
    else{
        //do_a = false;
        val = b[x][y];
    }

    let (_c, mmdata) = multiply_s(&mut a,&mut b);
    //println!("A: {:?}\nB: {:?}\nC: {:?}", a,b,_c);
    //println!("{}", val);
    let trace = mmdata.a_b.trace;
    //println!("{:?}", trace);
    let map = reuse_distance_eff(*trace,val);
    println!("{:#?}", map);
    Ok(())
}

fn test_stras() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let size = args[1].parse::<usize>().unwrap();

    let file_name = String::from("mm_s".to_string() + &args[1] + &".txt".to_string());
    //let freq_file_name = String::from("stras_freq_s".to_string() + &args[1] + &".txt".to_string());
    let mut output = File::create(file_name).unwrap();
    //let mut freq_output = File::create(freq_file_name).unwrap();

    // let mut rng = WyRand::new();
    let (mut a, mut b) = init(size);

    // let (C, _mmdata) = multiply(&mut A,&mut B);
    // println!("{:?}\n{:?}\n{:?}", A,B,C);



    let (_c, mmdata) = multiply(&mut a,&mut b);
    let a_b_dmd = mmdata.a_b.dmd;
    let cc_dmd = mmdata.c.dmd;
    let temp_dmd = mmdata.temp.dmd;
    let total_dmd = *a_b_dmd + *cc_dmd + *temp_dmd;
    // let a_b_dmd = *mmdata.a_b.dmd/mmdata.a_b.stack.len() as f32;
    // let cc_dmd = *mmdata.c.dmd/mmdata.c.stack.len() as f32;
    // let temp_dmd = *mmdata.temp.dmd/mmdata.temp.stack.len() as f32; 
    // let total_dmd = a_b_dmd + cc_dmd + temp_dmd;

    //println!("{:#?}", mmdata.a_b.freq_map);
    //println!("{:?}", _c);
    
    write!(
        output,
        "mat_size: {}x{}\ntotal: {}\nA_B: {}\nC: {}\ntemp: {}\n",
        size, size, total_dmd, a_b_dmd, cc_dmd, temp_dmd
    )?;
    
    // write!(
    //     freq_output,
    //     "{:?}\n{:?}\n{:?}\n",
    //     mmdata.a_b.freq_map, mmdata.c.freq_map,mmdata.temp.freq_map
    // )?;

    Ok(())
}

// fn test_mm(){
//     let args: Vec<String> = env::args().collect();
//     let size = args[1].parse::<usize>().unwrap();

//     //let file_name = String::from("mm_s".to_string() + &args[1] + &".txt".to_string());
//     let freq_file_name = String::from("mm_freq_s".to_string() + &args[1] + &".txt".to_string());
//     //let mut output = File::create(file_name).unwrap();
//     let mut freq_output = File::create(freq_file_name).unwrap();

//     // let mut rng = WyRand::new();
//     let (mut a, mut b) = init(size);

//     // let (C, _mmdata) = multiply(&mut A,&mut B);
//     // println!("{:?}\n{:?}\n{:?}", A,B,C);



//     let (_c, mmdata) = multiply(&mut a,&mut b);
//     let a_b_dmd = mmdata.a_b.dmd;
//     let cc_dmd = mmdata.c.dmd;
//     let temp_dmd = mmdata.temp.dmd;
//     let total_dmd = *a_b_dmd + *cc_dmd + *temp_dmd;

//     //println!("{:#?}", mmdata.a_b.freq_map);
//     // write!(
//     //     output,
//     //     "mat_size: {}x{}\ntotal: {}\nA_B: {}\nC: {}\ntemp: {}\n",
//     //     size, size, total_dmd, a_b_dmd, cc_dmd, temp_dmd
//     // )?;
//     write!(
//         freq_output,
//         "{:?}\n{:?}\n{:?}\n",
//         mmdata.a_b.freq_map, mmdata.c.freq_map,mmdata.temp.freq_map
//     )?;
// }


// fn test_qs() -> std::io::Result<()>{
//     let args: Vec<String> = env::args().collect();
//     let size = args[1].parse::<usize>().unwrap();
//     let mut reps = args[2].parse::<usize>().unwrap();

//     let file_name = String::from("s".to_string() + &args[1] + &".txt".to_string());
//     let mut output = File::create(file_name).unwrap();

//     let mut rng = WyRand::new();
//     let mut arr = init_ar(size);
//     let mut avg: f32 = 0.0;
//     let mut iteration = 1;

//     while reps > 1 {
//         let data: Data = qs(&mut arr);
//         rng.shuffle(&mut arr);
//         // let trace: Vec<String> = parse::build_trace("foo.log").unwrap();
//         // let trace_ref: Vec<&str> = trace.iter().map(AsRef::as_ref).collect();
//         let res = *data.dmd;

//         avg += res;

//         iteration += 1;
//         reps -= 1;
//     }
//     avg /= iteration as f32;

//     // rng.shuffle(&mut arr);
//     // let data = quick_sort_rt(&mut arr);
//     // println!("{}", data.dmd);

//     write!(
//         output,
//         "arr_size: {}\nreps: {}\navg: {}\n",
//         size, iteration, avg
//     )?;

//     Ok(())
// }
