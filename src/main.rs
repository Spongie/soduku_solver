use math::round;

struct Move {
    cell_x: i32,
    cell_y: i32,
    value: i32,
}

type SodukuBoard = [[i32; 9]; 9];

fn main() {
    let mut board = [
        [0, 6, 0, 2, 8, 4, 0, 0, 0],
        [0, 3, 1, 0, 0, 0, 0, 2, 4],
        [7, 2, 0, 3, 0, 0, 0, 9, 8],
        [0, 0, 0, 5, 4, 2, 0, 6, 0],
        [2, 5, 0, 0, 0, 0, 7, 4, 0],
        [4, 8, 0, 0, 0, 6, 0, 5, 9],
        [5, 4, 0, 1, 0, 3, 0, 7, 6],
        [0, 0, 0, 0, 9, 0, 4, 0, 0],
        [0, 0, 2, 0, 6, 0, 0, 0, 0],
    ];

    for m in get_available_moves(board) {
        println!("x:{} y:{} value:{}", m.cell_x, m.cell_y, m.value);
    }
}

fn get_values_on_row(board: SodukuBoard, row: usize) -> Vec<i32> {
    let mut moves = Vec::new();

    for x in 0..9 {
        moves.push(board[x][row]);
    }

    moves
}

fn get_values_on_column(board: SodukuBoard, column: usize) -> Vec<i32> {
    let mut moves = Vec::new();

    for y in 0..9 {
       // moves.push(board[column][y]);
    }

    moves
}

fn get_values_in_cell(board: SodukuBoard, column: i32, row: i32) -> Vec<i32> {
    let mut values = Vec::new();

    let column_start = round::ceil(column as f64 / 3.0, 0) as usize;
    let row_start = round::ceil(row as f64 / 3.0, 0) as usize;

    for x in column_start..column_start + 3 {
        for y in row_start..row_start + 3 {
            let value = board[x][y];

            if value > 0 {
                values.push(value);
            }
        }
    }

    values
}

fn get_available_moves(board: SodukuBoard) -> Vec<Move> {
    let mut moves = Vec::new();

    for x in 0..9 {
        for y in 0..9 {
            if board[x][y] > 0 {
                continue;
            }

            let values_in_cell = get_values_in_cell(board, x as i32, y as i32);
            let values_in_row = get_values_on_row(board, y);
            let values_in_column = get_values_on_column(board, x);

            let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let available_values: Vec<_> = values
                .iter()
                .filter(|value| !values_in_cell.contains(value) && !values_in_row.contains(value) && !values_in_column.contains(value))
                .collect();

            if available_values.len() == 1 {
                let value = *available_values[0];
                moves.push(Move{ cell_x: x as i32, cell_y: y as i32, value: value});
            }
        }
    }

    moves
}
