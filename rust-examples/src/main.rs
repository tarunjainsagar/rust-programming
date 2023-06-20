mod variables;

#[derive(Debug)]
enum MarkerType {
    Started,
    Ends,
}

fn main() {
    // Exercise 1 - Variables
    demarkers(MarkerType::Started, 1);
    variables::runall();
    demarkers(MarkerType::Ends, 1);
}

fn demarkers(marker: MarkerType, num: i32) {
    println!("==================");
    println!("Exercise No. {} {:?}", num, marker);
    println!("==================");
}
