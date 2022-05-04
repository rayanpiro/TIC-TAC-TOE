use super::table::{MATRIX_COL, MATRIX_ROW};

pub type PositionAxis = usize;
pub type Position = (PositionAxis, PositionAxis);
const TABLE_DIMENSION: Position = (MATRIX_COL as PositionAxis, MATRIX_ROW as PositionAxis);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InLineMoves {
    VERTICAL,
    HORIZONTAL,
    DIAGONALASC,
    DIAGONALDES,
}

impl InLineMoves {
    fn gen_all_linear_moves(
        &self,
        pos_vec: &mut Vec<Position>,
        visited_pos: &mut Vec<Position>,
    ) -> Vec<Position> {
        if pos_vec.len() == 0 {
            visited_pos.sort();
            visited_pos.dedup();
            return visited_pos.clone();
        }

        let position = &pos_vec[0];
        visited_pos.push(position.clone());

        let mut tmp: Vec<Position> = Moves::get_all_simple_moves(position)
            .into_iter()
            .filter(|m| self.filter_by_move(m))
            .filter_map(|m| {
                let pos = m.ret_index(position).unwrap();
                if visited_pos.iter().any(|vp| *vp == pos) {
                    return None;
                }
                Some(pos)
            })
            .collect();

        pos_vec.append(&mut tmp);

        self.gen_all_linear_moves(&mut pos_vec[1..].to_vec(), visited_pos)
    }

    pub fn get_all_linear_moves(&self, actual_position: &Position) -> Vec<Position> {
        if !Moves::is_inside_table(Some(actual_position.clone())) {
            return vec![];
        }
        self.gen_all_linear_moves(&mut vec![actual_position.clone()], &mut vec![])
    }

    fn filter_by_move(&self, m: &Moves) -> bool {
        match self {
            Self::VERTICAL => Self::is_vertical_move(m),
            Self::HORIZONTAL => Self::is_horizontal_move(m),
            Self::DIAGONALASC => Self::is_diagonalasc_move(m),
            Self::DIAGONALDES => Self::is_diagonaldes_move(m),
        }
    }

    fn is_diagonaldes_move(m: &Moves) -> bool {
        match m {
            Moves::Move(MovesX::LEFT, MovesY::UP) => true,
            Moves::Move(MovesX::RIGHT, MovesY::DOWN) => true,
            _ => false,
        }
    }

    fn is_diagonalasc_move(m: &Moves) -> bool {
        match m {
            Moves::Move(MovesX::LEFT, MovesY::DOWN) => true,
            Moves::Move(MovesX::RIGHT, MovesY::UP) => true,
            _ => false,
        }
    }

    fn is_horizontal_move(m: &Moves) -> bool {
        match m {
            Moves::Move(_, MovesY::NONE) => true,
            _ => false,
        }
    }

    fn is_vertical_move(m: &Moves) -> bool {
        match m {
            Moves::Move(MovesX::NONE, _) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MovesX {
    LEFT,
    RIGHT,
    NONE,
}

impl MovesX {
    fn ret_index(&self, actual_position: &Position) -> Option<PositionAxis> {
        match self {
            Self::LEFT if actual_position.0 > 0 => Some(actual_position.0 - 1),
            Self::RIGHT => Some(actual_position.0 + 1),
            Self::NONE => Some(actual_position.0),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MovesY {
    UP,
    DOWN,
    NONE,
}

impl MovesY {
    fn ret_index(&self, actual_position: &Position) -> Option<PositionAxis> {
        match self {
            Self::UP if actual_position.1 > 0 => Some(actual_position.1 - 1),
            Self::DOWN => Some(actual_position.1 + 1),
            Self::NONE => Some(actual_position.1),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Moves {
    Move(MovesX, MovesY),
}

impl Moves {
    fn is_inside_table(position: Option<Position>) -> bool {
        if position == None {
            return false;
        }
        let position = position.unwrap();
        position.0 < TABLE_DIMENSION.0 && position.1 < TABLE_DIMENSION.1
    }

    pub fn ret_index(&self, actual_position: &Position) -> Option<Position> {
        match self {
            Self::Move(x, y) => {
                Some((x.ret_index(actual_position)?, y.ret_index(actual_position)?))
            }
        }
    }

    pub fn get_all_simple_moves(actual_position: &Position) -> Vec<Self> {
        let all_moves: Vec<Self> = vec![
            Self::Move(MovesX::LEFT, MovesY::NONE),
            Self::Move(MovesX::LEFT, MovesY::UP),
            Self::Move(MovesX::LEFT, MovesY::DOWN),
            Self::Move(MovesX::RIGHT, MovesY::NONE),
            Self::Move(MovesX::RIGHT, MovesY::UP),
            Self::Move(MovesX::RIGHT, MovesY::DOWN),
            Self::Move(MovesX::NONE, MovesY::UP),
            Self::Move(MovesX::NONE, MovesY::DOWN),
        ];

        all_moves
            .into_iter()
            .filter(|m| Self::is_inside_table(m.ret_index(actual_position)))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod inlinemoves {
        use super::*;

        #[test]
        fn get_all_horizontal_moves_in_invalid_position() {
            let invalid_actual_position = (
                (MATRIX_COL + 1) as PositionAxis,
                (MATRIX_ROW + 1) as PositionAxis,
            );
            let moves = InLineMoves::HORIZONTAL.get_all_linear_moves(&invalid_actual_position);

            let possible_moves: Vec<Position> = vec![];

            assert_eq!(moves, possible_moves);
        }

        #[test]
        fn get_all_horizontal_moves_in_center() {
            let center = (
                (MATRIX_COL - 1) / 2 as PositionAxis,
                (MATRIX_ROW - 1) / 2 as PositionAxis,
            );
            let moves = InLineMoves::HORIZONTAL.get_all_linear_moves(&center);

            let mut possible_moves: Vec<Position> = vec![
                Moves::Move(MovesX::NONE, MovesY::NONE),
                Moves::Move(MovesX::LEFT, MovesY::NONE),
                Moves::Move(MovesX::RIGHT, MovesY::NONE),
            ]
            .iter()
            .map(|m| m.ret_index(&center).unwrap())
            .collect();

            possible_moves.sort();

            assert_eq!(moves, possible_moves);
        }

        #[test]
        fn get_all_vertical_moves_in_center() {
            let center = (
                (MATRIX_COL - 1) / 2 as PositionAxis,
                (MATRIX_ROW - 1) / 2 as PositionAxis,
            );
            let moves = InLineMoves::VERTICAL.get_all_linear_moves(&center);

            let mut possible_moves: Vec<Position> = vec![
                Moves::Move(MovesX::NONE, MovesY::NONE),
                Moves::Move(MovesX::NONE, MovesY::UP),
                Moves::Move(MovesX::NONE, MovesY::DOWN),
            ]
            .iter()
            .map(|m| m.ret_index(&center).unwrap())
            .collect();

            possible_moves.sort();

            assert_eq!(moves, possible_moves);
        }

        #[test]
        fn get_all_diagonaldes_moves_in_center() {
            let center = (
                (MATRIX_COL - 1) / 2 as PositionAxis,
                (MATRIX_ROW - 1) / 2 as PositionAxis,
            );
            let moves = InLineMoves::DIAGONALDES.get_all_linear_moves(&center);

            let mut possible_moves: Vec<Position> = vec![
                Moves::Move(MovesX::NONE, MovesY::NONE),
                Moves::Move(MovesX::LEFT, MovesY::UP),
                Moves::Move(MovesX::RIGHT, MovesY::DOWN),
            ]
            .iter()
            .map(|m| m.ret_index(&center).unwrap())
            .collect();

            possible_moves.sort();

            assert_eq!(moves, possible_moves);
        }

        #[test]
        fn get_all_diagonalasc_moves_in_center() {
            let center = (
                (MATRIX_COL - 1) / 2 as PositionAxis,
                (MATRIX_ROW - 1) / 2 as PositionAxis,
            );
            let moves = InLineMoves::DIAGONALASC.get_all_linear_moves(&center);

            let mut possible_moves: Vec<Position> = vec![
                Moves::Move(MovesX::NONE, MovesY::NONE),
                Moves::Move(MovesX::LEFT, MovesY::DOWN),
                Moves::Move(MovesX::RIGHT, MovesY::UP),
            ]
            .iter()
            .map(|m| m.ret_index(&center).unwrap())
            .collect();

            possible_moves.sort();

            assert_eq!(moves, possible_moves);
        }
    }

    mod moves {
        use super::*;
        #[test]
        fn get_all_possible_moves_in_invalid_position() {
            let invalid_actual_position = (
                (MATRIX_COL + 1) as PositionAxis,
                (MATRIX_ROW + 1) as PositionAxis,
            );
            let moves = Moves::get_all_simple_moves(&invalid_actual_position);

            let possible_moves: Vec<Moves> = vec![];

            assert_eq!(moves, possible_moves);
        }

        #[test]
        fn get_all_possible_moves_in_bottom_right_corner() {
            let actual_position = (
                (MATRIX_COL - 1) as PositionAxis,
                (MATRIX_ROW - 1) as PositionAxis,
            );
            let moves = Moves::get_all_simple_moves(&actual_position);

            let possible_moves = vec![
                Moves::Move(MovesX::LEFT, MovesY::NONE),
                Moves::Move(MovesX::LEFT, MovesY::UP),
                Moves::Move(MovesX::NONE, MovesY::UP),
            ];

            assert_eq!(moves, possible_moves);
        }

        #[test]
        fn get_all_possible_moves_in_the_top_left_corner() {
            let actual_position = (0, 0);
            let moves = Moves::get_all_simple_moves(&actual_position);

            let possible_moves = vec![
                Moves::Move(MovesX::RIGHT, MovesY::NONE),
                Moves::Move(MovesX::RIGHT, MovesY::DOWN),
                Moves::Move(MovesX::NONE, MovesY::DOWN),
            ];

            assert_eq!(moves, possible_moves);
        }
    }
}
