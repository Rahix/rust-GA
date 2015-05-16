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
// ( I am NOT good with ascii arts D: )
//
// (c) Rahix
extern crate rand;

use std::thread;
use rand::Rng;

fn clear_screen()
{
    print!("\x1B[2J \x1B[0;0f");
}

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

fn print_population(population: &Vec<Vec<i32> >)
{
    let i_individuums = population.len();
    for i_i in 0..i_individuums
    {
        let mut line = "".to_string();
        let i_chromosomes = population[i_i].len();
        let ref individuum = population[i_i];
        for i_c in 0..i_chromosomes
        {
            if individuum[i_c] == 1
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

fn mutation(individuum: &mut Vec<i32>)
{
    let random_point = rand::thread_rng().gen_range(0,80);
    individuum[random_point] = rand::thread_rng().gen_range(0,100) % 2;
}

fn get_indiviuum(population: &Vec<Vec<i32> >, better: bool) -> usize
{
    // Generate two random numbers,
    // if better is set, return the one with the better fitness,
    // if better is false, return the one with worse fitness.
    // This is used to simulate Darwins "Surviving of the fittest"
    let num1 = rand::thread_rng().gen_range(0, population.len());
    let num2 = rand::thread_rng().gen_range(0, population.len());
    if better == true
    {
        if fitness(&population[num1]) >= fitness(&population[num2])
        {
            return num1;
        }
        else
        {
            return num2;
        }
    }
    else
    {
        if fitness(&population[num1]) >= fitness(&population[num2])
        {
            return num2;
        }
        else
        {
            return num1;
        }
    }
}

fn simple_point_crossover_random(population: &mut Vec<Vec<i32> >)
{
    // Get two parents
    let p1 = get_indiviuum(population, true);
    let p2 = get_indiviuum(population, true);
    // And one child
    let child = get_indiviuum(population, false);
    // Get random crossover point
    let crossover_point = rand::thread_rng().gen_range(1, 79);
    // Copy everything until the crossoverpoint into child. (From p1)
    for i in 0..crossover_point
    {
        population[child][i] = population[p1][i];
    }
    // Copy everything after the crossoverpoint into child. (From p2)
    for i in crossover_point..80
    {
        population[child][i] = population[p2][i];
    }
    // Do a random mutation
    if rand::thread_rng().gen_range(0,100) % 2 == 1
    {
        mutation(&mut population[child]);
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
    loop
    {
        clear_screen();
        thread::sleep_ms(30);
        simple_point_crossover_random(&mut population);
        print_population(&population);
        println!("Running . . . \t\t \x1B[32;1mPress CTRL-C to exit!\x1B[37;0m")
    }
}
