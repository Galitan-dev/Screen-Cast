extern crate repng;
extern crate scrap;
extern crate hyper;
extern crate tokio;

mod screenshot;
mod stream;
mod ffplay;

const TEST: Test = Test::Stream;

#[tokio::main]
async fn main() { 
    match TEST {
        Test::Screenshot => screenshot::main(),
        Test::Stream => stream::main().await,
        Test::FFPlay => ffplay::main(),
    }
}

#[allow(unused)]
enum Test {
    Screenshot,
    Stream,
    FFPlay,
} 