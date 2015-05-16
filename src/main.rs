// The main file containing almost everything.
// This header has no purpose. It is only here because I need something to put here.
//
//                     T          GGG   AAA
//                     T         G   G A   A
// R RR  U   U  SSSS  TTT  ##### G     AAAAA
// RR  R U   U S       T         G GGG A   A
// R     U   U  SSS    T         G   G A   A
// R     U   U     S   T T       G   G A   A
// R      UUUU SSSS     T         GGG  A   A
//
//
// (Just for fun :)
//
// (c) Rahix
extern crate rand;

use std::io;
use rand::Rng;

fn fitness(individuum: &Vec<i32>) -> i32
{
    let mut i:i32 = 0;
    let mut fitness = 0;
    for i2 in 0..individuum.len()
    {
        let chromosome = individuum[i2];
        if chromosome == (i % 2)
        {
            fitness += 1;
        }
        i += 1;
    }
    fitness
}

fn print_population(population: Vec<Vec<i32> >)
{
    let i_individuums = population.len();
    for i_i in 0..i_individuums
    {
        let mut line = "".to_string();
        let i_chromosomes = population[i_i].len();
        let ref individuum = population[i_i];
        for i_c in 0..i_chromosomes
        {
            if(individuum[i_c] == 1)
            {
                line = line + &"#".to_string();
            }
            else
            {
                line = line + &" ".to_string();
            }
        }
        let fitness = fitness(individuum);
        println!("Individuum {}:\t{}\tFitness:{}", i_i.to_string(), line, fitness.to_string());
    }
}

fn main()
{
    let version = "0.0.1";
    println!("rust-GA version {}",version);
    println!("Initializing . . .");
    let mut population = vec![vec![0; 80]; 20];
    let i_chromosomes = 80;
    let i_individuums = 20;
    for i_i in 0..i_individuums
    {
        for i_c in 0..i_chromosomes
        {
            population[i_i][i_c] = rand::thread_rng().gen_range(0, 100) % 2;
        }
    }
    print_population(population);
}
