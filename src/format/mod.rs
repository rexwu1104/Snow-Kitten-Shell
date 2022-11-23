mod impls;

#[derive(Debug)]
pub struct Format {
    raw: String,
    command: Option<(usize, usize)>,
    args: Vec<(usize, usize)>,
    options: Vec<[(usize, usize); 2]>
}