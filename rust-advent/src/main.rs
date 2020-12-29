fn main() {
    println!("Hello, world!");
    let x = 1;
    {
        let y = 2;

        println!("X IS {} AND Y IS {}", x, y);
    }
}
