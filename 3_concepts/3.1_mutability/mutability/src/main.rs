fn main() {
    {
        let x = 5;
        println!("The value of x is: {}", x);
        // x = 6;
        println!("The value of x is: {}", x);
    }

    {
        let mut x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
    }

    {
        const MAX_POINTS : u32 = 10_000;
        println!("max points is: {}", MAX_POINTS)
    }

    {
        let x = 5;

        let x = x + 1;
    
        let x = x * 2;
    
        println!("The value of x is: {}", x);
    }

    {
        let spaces = "   ";
        let spaces = spaces.len();
        println!("spaces is: {}", spaces);
    }
}
