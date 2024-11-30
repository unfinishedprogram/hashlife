use std::collections::HashMap;

use super::{
    cell::{BaseCell, Cell, CompositeCell},
    cell_id::CellId,
    Life,
};

impl Life {
    pub fn from_cell_positions(max_depth: u8, points: Vec<(i32, i32)>) -> Self {
        let mut life = Self::new(max_depth);
        let base_alive_id = life.add_cell(Cell::Base(BaseCell::Alive));

        let min_x = *points.iter().map(|(x, _)| x).min().unwrap_or(&0);
        let min_y = *points.iter().map(|(_, y)| y).min().unwrap_or(&0);

        let mut nodes = HashMap::<(u64, u64), CellId>::from_iter(
            points
                .iter()
                .map(|(x, y)| ((x - min_x) as u64, (y - min_y) as u64))
                .map(|point| (point, base_alive_id)),
        );

        let mut level = 0;

        while nodes.len() > 1 {
            let mut next_level: HashMap<(u64, u64), CellId> = HashMap::new();

            while !nodes.is_empty() {
                let (x, y) = nodes.keys().next().unwrap();
                // Round down to the nearest even number
                let (x, y) = (x - (x & 1), y - (y & 1));

                // Since these coordinates are based off of a node that we know exists,
                // at least one of these quadrants must exist.
                let nw = nodes.remove(&(x, y)).unwrap_or(life.empty_of_layer(level));
                let ne = nodes
                    .remove(&(x + 1, y))
                    .unwrap_or(life.empty_of_layer(level));
                let sw = nodes
                    .remove(&(x, y + 1))
                    .unwrap_or(life.empty_of_layer(level));
                let se = nodes
                    .remove(&(x + 1, y + 1))
                    .unwrap_or(life.empty_of_layer(level));

                next_level.insert((x >> 1, y >> 1), life.join(nw, ne, sw, se));
            }

            level += 1;
            nodes = next_level;
        }

        let root = nodes.values().next().unwrap();
        life.root = *root;
        life
    }

    pub fn cell_positions(&self, min_depth: u8) -> Vec<(i32, i32)> {
        let mut positions = Vec::new();
        let offset = (1 << (self.root.layer() as i32 - min_depth as i32)) / 2;
        self.unpack_cells(min_depth, &mut positions, self.root, (-offset, -offset));
        positions
    }

    fn unpack_cells(
        &self,
        max_depth: u8,
        cells: &mut Vec<(i32, i32)>,
        cell: CellId,
        (x, y): (i32, i32),
    ) {
        if cell.layer() <= max_depth as usize {
            if cell.alive() > 0 {
                cells.push((x, y));
            }
            return;
        }

        let cell = self.get_cell(cell).unwrap();
        match cell {
            Cell::Base(BaseCell::Alive) => {
                cells.push((x, y));
            }
            Cell::Base(BaseCell::Dead) => {}
            Cell::Composite(CompositeCell { nw, ne, sw, se, .. }) => {
                let half = 1 << (cell.depth() - 1 - max_depth);
                if nw.alive() > 0 {
                    self.unpack_cells(max_depth, cells, *nw, (x, y));
                }
                if ne.alive() > 0 {
                    self.unpack_cells(max_depth, cells, *ne, (x + half, y));
                }
                if sw.alive() > 0 {
                    self.unpack_cells(max_depth, cells, *sw, (x, y + half));
                }
                if se.alive() > 0 {
                    self.unpack_cells(max_depth, cells, *se, (x + half, y + half));
                }
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_pack_unpack() {
        let points = vec![(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (31, 7)];
        let life = Life::from_cell_positions(8, points.clone());
        let unpacked = life.cell_positions(8);

        let expected: HashSet<(i32, i32)> = HashSet::from_iter(points);
        let actual = HashSet::from_iter(unpacked);
        assert_eq!(expected, actual);
    }
}
