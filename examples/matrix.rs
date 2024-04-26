use anyhow::Result;
use concurrency::Matrix;

// [[1,2],[1,2],[1,2]] => [1,2,1,2,1,2]

fn main() -> Result<()> {
    let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
    let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
    let c = a * b;

    println!("{:#?}", c);

    Ok(())
}
