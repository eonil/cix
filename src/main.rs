use structopt::StructOpt;

mod cix;

fn main() {
    println!("Hello, world!");
}

#[derive(StructOpt)]
#[structopt(name="cix", about="CI tools collection.")]
enum Command {
    Apple(cix::apple::Apple),
}

