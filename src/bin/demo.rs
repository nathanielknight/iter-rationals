use iter_rationals::Rationals;

fn main() {
    let rs = Rationals::<u32>::new();

    for r in rs.take(20) {
        println!("{r}");
    }
}
