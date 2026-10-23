mod board;

fn main() {
    let mut x = board::Grid::new(10, 20);
    println!("{:?}", x);
    x.matrix.row.push(board::Cell::new());
    x.print();
}
