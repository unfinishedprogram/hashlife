use crate::life::cell::Cell;

use super::{cell_id::CellId, Life};

impl Life {
    pub fn join(&mut self, nw: CellId, ne: CellId, sw: CellId, se: CellId) -> CellId {
        debug_assert_eq!(nw.layer(), ne.layer());
        debug_assert_eq!(nw.layer(), sw.layer());
        debug_assert_eq!(nw.layer(), se.layer());

        self.add_cell(Cell::composite(nw, ne, sw, se))
    }

    pub fn centered_subnode(&mut self, id: CellId) -> CellId {
        let cell = self.get_cell(id).unwrap().as_composite();

        let nw = self.get_cell(cell.nw).unwrap().as_composite().se;
        let ne = self.get_cell(cell.ne).unwrap().as_composite().sw;
        let sw = self.get_cell(cell.sw).unwrap().as_composite().ne;
        let se = self.get_cell(cell.se).unwrap().as_composite().nw;

        self.join(nw, ne, sw, se)
    }

    pub fn centered_sub_subnode(
        &mut self,
        nw: CellId,
        ne: CellId,
        sw: CellId,
        se: CellId,
    ) -> CellId {
        assert!(nw.layer() > 0);

        let nw = self
            .get_cell(self.get_cell(nw).unwrap().as_composite().se)
            .unwrap()
            .as_composite()
            .se;
        let ne = self
            .get_cell(self.get_cell(ne).unwrap().as_composite().sw)
            .unwrap()
            .as_composite()
            .sw;
        let sw = self
            .get_cell(self.get_cell(sw).unwrap().as_composite().ne)
            .unwrap()
            .as_composite()
            .ne;
        let se = self
            .get_cell(self.get_cell(se).unwrap().as_composite().nw)
            .unwrap()
            .as_composite()
            .nw;

        self.join(nw, ne, sw, se)
    }

    pub fn centered_horizontal(&mut self, w: CellId, e: CellId) -> CellId {
        let w = self.get_cell(w).unwrap().as_composite();
        let e = self.get_cell(e).unwrap().as_composite();

        self.join(
            self.get_cell(w.ne).unwrap().as_composite().se,
            self.get_cell(e.nw).unwrap().as_composite().sw,
            self.get_cell(w.se).unwrap().as_composite().ne,
            self.get_cell(e.sw).unwrap().as_composite().nw,
        )
    }

    pub fn centered_vertical(&mut self, n: CellId, s: CellId) -> CellId {
        let n = self.get_cell(n).unwrap().as_composite();
        let s = self.get_cell(s).unwrap().as_composite();

        self.join(
            self.get_cell(n.sw).unwrap().as_composite().se,
            self.get_cell(n.se).unwrap().as_composite().sw,
            self.get_cell(s.nw).unwrap().as_composite().ne,
            self.get_cell(s.ne).unwrap().as_composite().nw,
        )
    }
}
