use super::markers::Markers;

pub const MATRIX_COL: usize = 3;
pub const MATRIX_ROW: usize = MATRIX_COL;

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    table: [Markers; MATRIX_COL * MATRIX_ROW],
}

impl Table {
    fn new() -> Self {
        let table: [Markers; MATRIX_COL * MATRIX_ROW] =
            [Markers::NotMarked; MATRIX_COL * MATRIX_ROW];
        Self { table }
    }

    fn translate_from_multi_to_unidimensional(x: usize, y: usize) -> usize {
        y * MATRIX_ROW + x
    }

    pub fn get(&self, x: usize, y: usize) -> &Markers {
        self.table
            .get(Self::translate_from_multi_to_unidimensional(x, y))
            .expect("The position requested is out of the table dimensions")
    }

    pub fn count(&self, mark: Markers) -> usize {
        self.table.iter().filter(|&&m| m == mark).count()
    }

    pub fn from_string(input: &[&str]) -> Self {
        let mut table = Self::new();
        let mut vector: Vec<Markers> = Vec::new();

        for row in input {
            let mut tmp: Vec<Markers> = row
                .chars()
                .map(|char| match char {
                    'X' | 'x' => Markers::X,
                    'O' | 'o' => Markers::O,
                    _ => Markers::NotMarked,
                })
                .collect();

            vector.append(&mut tmp);
        }
        table.table = vector
            .try_into()
            .expect("The input doenst fit the table dimensions!");
        table
    }

    #[cfg(test)]
    fn all_x() -> Self {
        let mut table = Self::new();
        table.table.fill(Markers::X);
        table
    }

    #[cfg(test)]
    fn all_o() -> Self {
        let mut table = Self::new();
        table.table.fill(Markers::O);
        table
    }

    #[cfg(test)]
    fn mixed() -> Self {
        let end_of_first_row = Self::translate_from_multi_to_unidimensional(MATRIX_COL-1, 0);
        let init_of_last_row = Self::translate_from_multi_to_unidimensional(0, MATRIX_ROW-1);
        let end_of_last_row = Self::translate_from_multi_to_unidimensional(MATRIX_COL-1, MATRIX_ROW-1);

        let mut table = Self::new();
        table.table[0..=end_of_first_row].fill(Markers::X);
        table.table[init_of_last_row..=end_of_last_row].fill(Markers::O);
        table
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_string_all_x_test() {
        let init_string: Vec<&str> = vec![vec!["X"; MATRIX_COL]; MATRIX_ROW]
            .into_iter()
            .flatten()
            .collect();
        assert_eq!(Table::from_string(&init_string), Table::all_x())
    }

    #[test]
    fn from_string_all_o_test() {
        let init_string: Vec<&str> = vec![vec!["O"; MATRIX_COL]; MATRIX_ROW]
            .into_iter()
            .flatten()
            .collect();
        assert_eq!(Table::from_string(&init_string), Table::all_o())
    }

    #[test]
    fn from_string_mixed_test() {

        let init_string: Vec<&str> = vec![vec![" "; MATRIX_COL]; MATRIX_ROW-2]
            .into_iter()
            .flatten()
            .collect();
        let x_row = vec!["X"; MATRIX_COL];
        let o_row = vec!["O"; MATRIX_COL];

        let init_string = vec![x_row, init_string].concat();
        let init_string = vec![init_string, o_row].concat();

        // init_string[0] = x_row
        assert_eq!(Table::from_string(&init_string), Table::mixed())
    }
}
