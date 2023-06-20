mod datatypes;
mod variables;

#[derive(Debug)]
enum MarkerType {
    Started,
    Ends,
}

fn main() {
    let mut num: i8 = 1;

    // Exercise 1 - Variables
    demarkers(MarkerType::Started, num);
    variables::runall();
    demarkers(MarkerType::Ends, num);

    num += 1;
    // Exercise 2 - DataTypes
    demarkers(MarkerType::Started, num);
    datatypes::runall();
    demarkers(MarkerType::Ends, num);
}

fn demarkers(marker: MarkerType, num: i8) {
    println!("==================");
    println!("Exercise No. {} {:?}", num, marker);
    println!("==================");
}
