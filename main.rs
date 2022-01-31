use std::{thread, time};
use text_io::read;

fn main() {
    println!("Total Amount in $");
    let amount :f64 = read!();
    println!("Average APR in %");
    let apr :f64= read!();
    println!("how many blocks per rpc pull");
    let blocks :i64 = read!();

    let seconds_per_year = 31557600.0;
    let blocks_seconds = 2; //average 2 seconds per block;
    let blocks_timeout =  blocks * blocks_seconds;

    let timing = apr / 100.0 / seconds_per_year / 10.0;
    let amounty = amount * timing * 0.995; //increase of rewards every 0.1 seconds. Also giving it a small downplay value to never exceed the actual value.
    
    //Alternatively, you can set amounty equal to the last received reward, divided by 20. 
    //let amounty = last_reward_rpc_data / block_seconds / 10;

    let mut rewards = 0.0;
    let mut rpcvalue = 0.0;
    let mut rpc_call_counter = 0;
    let mut n = 0;
    loop {
        
        if rpc_call_counter == n {
            n += blocks_timeout * 10;
            println!("Checked in with rpc");
            //pull data from rpc
            //reward = last_reward_rpc_data;
            rewards = rpcvalue;
            println!("new value {}", rewards);
        } else {
            rewards += amounty;
            println!("{:.8}", rewards);
        }
        thread::sleep(time::Duration::from_millis(100));
        rpc_call_counter += 1;
        rpcvalue += amounty *1.005; //this is normal data pulled from rpc, removed here the dirt we added above.
    };


}
