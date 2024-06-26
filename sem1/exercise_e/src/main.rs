struct Solution {
    num_rows: i32,
}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows_conv: usize = num_rows as usize;
        if num_rows == 0 {
            return vec![vec![1].to_vec()].to_vec();
        }

        let mut triangle: Vec<Vec<i32>> = Vec::new();

        triangle.push(vec![1].to_vec());

        // Subsequent rows
        for row in 1..num_rows_conv {
            let mut current_row: Vec<i32> = Vec::new();

            // First element of each row is always 1
            current_row.push(1);

            // Calculate remaining elements using the previous row
            for col in 1..=row {
                let prev_row_left = triangle[row - 1][col - 1];
                let prev_row_right = if col == row {
                    0
                } else {
                    triangle[row - 1][col]
                };
                current_row.push(prev_row_left + prev_row_right);
            }

            triangle.push(current_row);
        }

        triangle
    }
}

fn main() {
    println!("{:#?}", Solution::generate(7));
}
