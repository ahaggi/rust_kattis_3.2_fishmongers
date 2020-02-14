use std::io;

fn main() {
    // 4 3       The first line of input contains two integers n (1≤n≤100000), the number of fish you have to sell, and m (1≤m≤100000), the number of fishmongers.
    // 1 2 7 5   n space-separated integers w1,w2,…,wn, the weight of each of your fish in kilograms (1≤wi≤100000)

    // 2 4       xj (1≤xj≤100000) and pj (1≤pj≤100000), respectively indicating how many fish the j’th fishmonger wants to buy and how many monies he will pay per kilogram.
    // 1 5
    // 3 3
    let stdin = io::stdin();

    let mut val0 = String::new();
    stdin.read_line(&mut val0).expect("Failed to read line");
    let mut fst_line:Vec<u32> = val0.trim().split_whitespace().map(|num| num.parse::<u32>().unwrap()).collect();
    let mut nr_of_fishmongers:u32 = fst_line.pop().unwrap();

    let mut val = String::new();
    stdin.read_line(&mut val).expect("Failed to read line");// reading the number of sheets he has of each paper size starting with A2 and ending with An.

    // let mut fishs_wight: Vec<u32> = vec![1, 2, 7, 5];
    let mut fishs_wight: Vec<u32> = val.trim().split_whitespace().map(|num| num.parse().unwrap()).collect();


    // let mut fishmongers_orders: Vec<(u32, u32)> = vec![(2, 4), (1, 5), (3, 3)];
    let mut fishmongers_orders: Vec<(u32, u32)> = vec![];


    while nr_of_fishmongers > 0{
        let mut line = String::new();
        stdin.read_line(&mut line).expect("Failed to read line");

        let nums: Vec<u32> = line
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

        fishmongers_orders.push((nums[0], nums[1]));
        nr_of_fishmongers -= 1; 
    }
    
    fishmongers_orders.sort_by(|a, b| a.1.cmp(&b.1));
    fishs_wight.sort();


    let mut sum:u64 = 0;
    while !fishmongers_orders.is_empty() && !fishs_wight.is_empty() {
        let order = fishmongers_orders.pop().unwrap();

        let mut nr_of_fishs = order.0;
        let price_per_kilo = order.1;

        while !fishs_wight.is_empty() && nr_of_fishs > 0 {
            sum = sum + (fishs_wight.pop().unwrap() as u64 * price_per_kilo as u64);
            nr_of_fishs -= 1;
        }
    }
    println!("{}", sum);
}
