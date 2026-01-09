
fn main() {
    let mut v = vec![1];
    for i in 2..1000 {
        v.push(i);
        if v.len() % 4 == 1 || v.len() % 4 == 3 {
            continue;
        }
        let acc = v.iter().fold(0, |acc, x| acc ^ *x);

        

        println!("{:?} {:?}", v.len() ,acc);
    }
}
