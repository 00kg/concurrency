use anyhow::{anyhow, Result};

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }

    // pub fn len(&self) -> usize {
    //     self.data.len()
    // }

    // pub fn iter(&self) -> std::slice::Iter<T> {
    //     self.data.iter()
    // }
}

// 第一种方式，实现可以通过下标进行索引
// use std::ops::Index;
// impl<T> Index<usize> for Vector<T> {
//     type Output = T;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.data[index]
//     }
// }

// 注意，在第一种方式中，我们一共要实现 len方法，iter方法，Index trait
// 但如果实现Deref的话都可以省略，Rust会自动对数据进行Deref（Vector<T>.len 由于Vector<T>没有len方法，Rust会自动进行Deref，从而获取到&self.data，这个数据本身就是有len方法的）
use std::ops::{Add, AddAssign, Deref, Mul};
impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        return Err(anyhow!("Dot product error: a.len != b.len"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    Ok(sum)
}
