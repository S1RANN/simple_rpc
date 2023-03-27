use client::MathService;

fn main() {
    let mut service = MathService::new("localhost:8080");
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let a = parts.next().unwrap().parse::<i32>().unwrap();
        let b = parts.next().unwrap().parse::<i32>().unwrap();
        let op = parts.next().unwrap();
        let result = match op {
            "+" => service.add(a, b),
            "-" => service.sub(a, b),
            "*" => service.mul(a, b),
            "/" => service.div(a, b),
            _ => panic!("Invalid operator"),
        };
        println!("{} {} {} = {}", a, op, b, result);
    }
}
