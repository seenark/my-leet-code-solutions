function diagonalSum(mat: number[][]): number {
  if (mat.length === 1) return mat[0][0];

  let sum = 0;
  for (let row = 0, col = mat[0].length - 1; row < mat.length; row++, col--) {
    if (row !== col) {
      sum += mat[row][col];
    }
    sum += mat[row][row];
    console.log(row, col, sum);
  }
  return sum;
}

const mat = [
  [7, 3, 1, 9],
  [3, 4, 6, 9],
  [6, 9, 6, 6],
  [9, 5, 8, 5],
];

const s = diagonalSum(mat);
console.log(s);
