use communicator::network::server;
use communicator::network::client;
use communicator::network::math::{add, subtract};
// mod utilities {
//     pub fn print_hello() {
//         println!("Hello!");
//     }

//     pub fn print_goodbye() {
//         println!("Goodbye!");
//     }
// }
// use utilities::{print_hello, print_goodbye};


mod utilities {
    pub fn print_message(){
        println!("better than JS");
    }
}

use utilities::print_message as pm;


fn main() {
    server::start_server();
    client::connect_client();

    let sum = add(5,3);
    let difference = subtract(5, 3);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);

    pm();

}