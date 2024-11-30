use std::collections::HashMap;

use super::{
    cell::{BaseCell, Cell, CompositeCell},
    cell_id::CellId,
    Life,
};

#[derive(Clone, Copy)]
pub struct CellBounds {
    pub min_x: i64,
    pub min_y: i64,
    pub max_x: i64,
    pub max_y: i64,
}

impl Life {
    pub fn from_cell_positions(max_depth: u8, points: Vec<(i64, i64)>) -> Self {
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

    pub fn cell_positions(&self, min_depth: u8, bounds: CellBounds) -> Vec<(i64, i64)> {
        let half_width = 1 << (self.root.layer() - 1 - min_depth as usize);
        let mut positions = Vec::new();

        positions.push((0, 0));

        let bound_div = 1 << min_depth as i64;

        let mapped_bounds = CellBounds {
            min_x: bounds.min_x / bound_div,
            min_y: bounds.min_y / bound_div,
            max_x: bounds.max_x / bound_div,
            max_y: bounds.max_y / bound_div,
        };

        self.unpack_cells(
            min_depth,
            &mut positions,
            self.root,
            (-half_width, -half_width),
            mapped_bounds,
        );
        positions
    }

    fn unpack_cells(
        &self,
        max_depth: u8,
        cells: &mut Vec<(i64, i64)>,
        cell: CellId,
        (x, y): (i64, i64),
        bounds: CellBounds,
    ) {
        let width = 1 << (cell.layer() - max_depth as usize);
        let half_width = width >> 1;

        if x > bounds.max_x
            || y > bounds.max_y
            || x + width < bounds.min_x
            || y + width < bounds.min_y
        {
            return;
        }

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
                if nw.alive() > 0 {
                    self.unpack_cells(max_depth, cells, *nw, (x, y), bounds);
                }
                if ne.alive() > 0 {
                    self.unpack_cells(max_depth, cells, *ne, (x + half_width, y), bounds);
                }
                if sw.alive() > 0 {
                    self.unpack_cells(max_depth, cells, *sw, (x, y + half_width), bounds);
                }
                if se.alive() > 0 {
                    self.unpack_cells(
                        max_depth,
                        cells,
                        *se,
                        (x + half_width, y + half_width),
                        bounds,
                    );
                }
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use std::{collections::HashSet, i64};

    #[test]
    fn test_pack_unpack() {
        let points = vec![(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (31, 7)];
        let life = Life::from_cell_positions(8, points.clone());
        let unpacked = life.cell_positions(
            8,
            CellBounds {
                min_x: i64::MIN,
                min_y: i64::MIN,
                max_x: i64::MAX,
                max_y: i64::MAX,
            },
        );

        let expected: HashSet<(i64, i64)> = HashSet::from_iter(points);
        let actual = HashSet::from_iter(unpacked);
        assert_eq!(expected, actual);
    }
}
