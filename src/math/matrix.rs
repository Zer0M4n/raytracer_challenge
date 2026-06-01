struct Matrix {
    y: usize,
    x: usize,
}

impl Matrix {
    fn new(x: usize , y: usize) -> Self{
        Matrix { x, y }
    }   
}

#[cfg(test)]
mod test{
    use super::*;
}