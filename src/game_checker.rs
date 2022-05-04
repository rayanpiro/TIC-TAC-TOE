use super::markers::Markers;
use super::moves::InLineMoves;
use super::table::{Table, MATRIX_COL, MATRIX_ROW};
use std::cmp::max;

const MARKS_TO_WIN: usize = MATRIX_COL;
pub struct GameChecker;

impl GameChecker {
    pub fn check_table(table: &Table) -> String {
        let mut max_num_x_inline = 0;
        let mut max_num_o_inline = 0;

        // The core positions to check are 3 of 4 corners and center if we are in a 3x3 game
        let core_positions = vec![
            // Top left corner
            (0, 0),
            // Top right corner
            (MATRIX_COL - 1, 0),
            // Bottom left corner
            (0, MATRIX_ROW - 1),
            // Center
            ((MATRIX_COL - 1) / 2, (MATRIX_ROW - 1) / 2),
        ];

        let inliners = vec![
            InLineMoves::VERTICAL,
            InLineMoves::HORIZONTAL,
            InLineMoves::DIAGONALASC,
            InLineMoves::DIAGONALDES,
        ];

        for position in core_positions {
            let mark = table.get(position.0, position.1);

            if *mark == Markers::NotMarked {
                continue;
            }

            for inline in inliners
                .iter()
                .map(|iln| iln.get_all_linear_moves(&position))
            {
                let count = inline
                    .iter()
                    .filter(|p| table.get(p.0, p.1) == mark)
                    .count();

                match *mark {
                    Markers::X => max_num_x_inline = max(max_num_x_inline, count),
                    Markers::O => max_num_o_inline = max(max_num_o_inline, count),
                    Markers::NotMarked => unreachable!(),
                };
            }
        }

        let num_of_x = table.count(Markers::X);
        let num_of_o = table.count(Markers::O);

        if max_num_x_inline == MARKS_TO_WIN && num_of_x == num_of_o + 1 {
            return "X".to_string();
        } else if max_num_o_inline == MARKS_TO_WIN
            && num_of_x == num_of_o
            && max_num_x_inline != MARKS_TO_WIN
        {
            return "O".to_string();
        }

        if (MATRIX_COL * MATRIX_ROW % 2 == 0 && num_of_x == num_of_o)
            || (MATRIX_COL * MATRIX_ROW % 2 != 0 && num_of_x == num_of_o + 1)
        {
            return "EMPATE".to_string();
        }

        "NULO".to_string()
    }
}
