#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed_matrix = [[0; 3]; 3];

    for col_index in 0..3 {
        for row_index in 0..3 {
            transposed_matrix[row_index][col_index] = matrix[col_index][row_index];
        }
    }

    transposed_matrix
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    matrix.iter().for_each(|row_items| {
        let formatted_row = row_items
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        println!("[{formatted_row}],");
    })
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
