fn diagonal_sum1(mat: Vec<Vec<i32>>) -> i32 {
    let mut sum: i32 = 0;
    let mut col_index = mat[0].len() - 1;
    mat.iter().enumerate().for_each(|(row_index, row)| {
        println!("{:?} => rowIndex:{row_index}, colIndex:{col_index}", row);
        if row_index != col_index {
            sum += mat[row_index][col_index];
        }
        sum += mat[row_index][row_index];
        println!("iter sum: {sum}");
        col_index -= 1;
    });
    sum
}
fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let mut sum: i32 = 0;
    let mut col_index: usize = mat.len() - 1;
    for row_index in 0..mat.len() {
        println!("rowIndex:{row_index}, colIndex:{col_index}");
        if row_index != col_index {
            sum += mat[row_index][col_index];
        }
        sum += mat[row_index][row_index];
        println!("iter sum: {sum}");
        col_index = col_index.wrapping_sub(1);
    }
    sum
}

pub fn run() {
    let mat = vec![
        vec![7, 3, 1, 9],
        vec![3, 4, 6, 9],
        vec![6, 9, 6, 6],
        vec![9, 5, 8, 5],
    ];
    // let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let sum = diagonal_sum(mat);
    println!("{sum}")
}
