use server::MathService;
fn main() {
    let service = MathService::new("localhost:8080");
    service.run();
}
