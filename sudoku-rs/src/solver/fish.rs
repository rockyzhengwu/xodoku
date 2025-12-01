use std::collections::HashSet;

use crate::{
    candidate::Candidate,
    grid::{Grid, HouseType},
    grid_constant::{block, col, row},
    solver::{SolverStrategy, step::Step, step_accumulator::StepAccumulator},
    util::{create_permutations, indexset::IndexSet},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Fish {
    remove_candidates: Vec<Candidate>,
    highlight_candidates: Vec<Candidate>,
    fins: Vec<Candidate>,
    sashimis: Vec<Candidate>,
    basics: Vec<u8>,
    covers: Vec<u8>,
    value: u8,
}

pub struct FishFinder {
    degree: u8,
    finned: bool,
    sashimi: bool,
}

// basic set 中多出来的不再 cover set 里的是 fin，一次有且只能在 一个 basic house
// 出现，且必须在同一个 block, 里
//
// 每一个 cover house 必须和 至少两个 baseic 有交集，才能成为一个 fish ,或者 一个 finned fish,
// 如果有一个 cover 只和一个basic 有交集，但缺少的那个 block 有 fin 也可以组成 sashimi
impl FishFinder {
    pub fn new(degree: u8, finned: bool, sashimi: bool) -> Self {
        Self {
            degree,
            finned,
            sashimi,
        }
    }
    pub fn find_fish(
        &self,
        basic: HouseType,
        cover: HouseType,
        grid: &Grid,
        acc: &mut dyn StepAccumulator,
    ) {
        for value in 1..=9 {
            let basics: Vec<u8> = basic
                .houses()
                .into_iter()
                .filter(|h| {
                    let cells = grid.pential_cells_in_house(*h, value);
                    !cells.is_empty() && cells.count() <= self.degree + 3
                })
                .collect();

            let covers: Vec<u8> = cover
                .houses()
                .into_iter()
                .filter(|h| {
                    let cells = grid.pential_cells_in_house(*h, value);
                    !cells.is_empty()
                })
                .collect();

            let basic_permutation = create_permutations(basics, self.degree);
            let covers_permutation = create_permutations(covers, self.degree);

            for basic_permu in basic_permutation {
                let basic_cells = basic_permu
                    .iter()
                    .map(|h| grid.pential_cells_in_house(*h, value))
                    .fold(IndexSet::new_empty(), |u, s| u.union(&s));

                for cover_permu in covers_permutation.iter() {
                    let cover_sets: Vec<IndexSet> = cover_permu
                        .iter()
                        .map(|h| grid.pential_cells_in_house(*h, value))
                        .collect();
                    let cover_cells = cover_sets
                        .iter()
                        .fold(IndexSet::new_empty(), |u, s| u.union(s));
                    let cross_cells = basic_cells.intersect(&cover_cells);

                    let cross_num: Vec<u8> = cover_sets
                        .iter()
                        .map(|cs| cs.intersect(&cross_cells).count())
                        .collect();
                    if cross_num.contains(&0) {
                        continue;
                    }
                    let one_count = cross_num
                        .iter()
                        .fold(0, |c, n| if *n == 1 { c + 1 } else { c });
                    if one_count > 1 {
                        continue;
                    }
                    let fins = basic_cells.difference(&cover_cells);

                    if !self.finned && !self.sashimi && one_count == 0 && fins.is_empty() {
                        // this is baic fish
                        let remove_cells = cover_cells.difference(&basic_cells);
                        if remove_cells.is_empty() {
                            continue;
                        }
                        let remove_candidates: Vec<Candidate> = remove_cells
                            .iter()
                            .map(|cell| Candidate::new(cell, value))
                            .collect();
                        let highlight_candidates: Vec<Candidate> = cross_cells
                            .iter()
                            .map(|cell| Candidate::new(cell, value))
                            .collect();
                        let fish = Fish {
                            remove_candidates,
                            highlight_candidates,
                            basics: basic_permu.clone(),
                            covers: cover_permu.clone(),
                            fins: Vec::new(),
                            sashimis: Vec::new(),
                            value: value,
                        };
                        if acc.add_step(Step::Fish(fish)) {
                            return;
                        }
                        continue;
                    }

                    let fin_blocks: HashSet<u8> = fins.iter().map(|cell| block(cell)).collect();

                    if fin_blocks.len() != 1 {
                        continue;
                    }

                    let fin_block = fin_blocks.iter().next().unwrap().to_owned();
                    let fin_houses: HashSet<u8> = match basic {
                        HouseType::Row => fins.iter().map(|cell| row(cell)).collect(),
                        HouseType::Column => fins.iter().map(|cell| col(cell)).collect(),
                        HouseType::Block => panic!("invaid basics in fish"),
                    };
                    if fin_houses.len() != 1 {
                        continue;
                    }
                    let fin_house = fin_houses.iter().next().unwrap().to_owned();

                    //println!(
                    //    "{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",
                    //    value,
                    //    fin_block,
                    //    fins.values(),
                    //    basic_permu,
                    //    cover_permu,
                    //    fin_houses,
                    //    one_count,
                    //    cross_num,
                    //    cover_sets[3].values(),
                    //);
                    if self.finned && one_count == 0 && !fins.is_empty() {
                        // fined finned fish, make sure fin house and
                        let fin_visiable_cells = grid.pential_cells_in_house(fin_block, value);
                        let remove_cells = cover_cells
                            .difference(&cross_cells)
                            .intersect(&fin_visiable_cells);
                        if remove_cells.is_empty() {
                            continue;
                        }
                        let remove_candidates: Vec<Candidate> = remove_cells
                            .iter()
                            .map(|cell| Candidate::new(cell, value))
                            .collect();
                        let highlight_candidates: Vec<Candidate> = cross_cells
                            .iter()
                            .map(|cell| Candidate::new(cell, value))
                            .collect();
                        let fin_candidates: Vec<Candidate> = fins
                            .iter()
                            .map(|cell| Candidate::new(cell, value))
                            .collect();
                        let fish = Fish {
                            remove_candidates,
                            highlight_candidates,
                            fins: fin_candidates,
                            basics: basic_permu.clone(),
                            covers: cover_permu.clone(),
                            value,
                            sashimis: Vec::new(),
                        };
                        if acc.add_step(Step::Fish(fish)) {
                            return;
                        }
                    }

                    if self.sashimi && !fins.is_empty() {
                        let fin_house_cells = grid.pential_cells_in_house(fin_house, value);
                        if fin_house_cells.difference(&fins).count() != 1 {
                            continue;
                        }

                        let fin_visiable_cells = grid.pential_cells_in_house(fin_block, value);
                        let remove_cells = cover_cells
                            .difference(&cross_cells)
                            .intersect(&fin_visiable_cells);

                        if remove_cells.is_empty() {
                            continue;
                        }
                        let remove_candidates: Vec<Candidate> = remove_cells
                            .iter()
                            .map(|cell| Candidate::new(cell, value))
                            .collect();
                        let highlight_candidates: Vec<Candidate> = cross_cells
                            .iter()
                            .map(|cell| Candidate::new(cell, value))
                            .collect();
                        let fin_candidates: Vec<Candidate> = fins
                            .iter()
                            .map(|cell| Candidate::new(cell, value))
                            .collect();
                        let fish = Fish {
                            remove_candidates,
                            highlight_candidates,
                            fins: fin_candidates,
                            basics: basic_permu.clone(),
                            covers: cover_permu.clone(),
                            value,
                            sashimis: Vec::new(),
                        };
                        if acc.add_step(Step::Fish(fish)) {
                            return;
                        }
                    }
                }
            }
        }
    }
}

impl SolverStrategy for FishFinder {
    fn find_step(&self, grid: &Grid, acc: &mut dyn StepAccumulator) {
        self.find_fish(HouseType::Row, HouseType::Column, grid, acc);
        if acc.is_finish() {
            return;
        }
        self.find_fish(HouseType::Column, HouseType::Row, grid, acc);
    }
}
#[cfg(test)]
mod test {
    use crate::{
        grid::Grid,
        solver::{
            SolverStrategy, fish::FishFinder, step::Step, step_accumulator::AllStepAccumulator,
        },
    };

    #[test]
    fn test_finned_xwing() {
        let s = ":0310:9:.+52+6+7.3.+8.3...+5+6+2767..+3+2+5.+1+2+8...61.+5.+6....+2.47+1+452+3+86+9+82+73+149+5+6.9.+2+67+48+3+3+469+58+71+2::933:r24 c35 fr2c1";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = FishFinder::new(2, true, false);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }

    #[test]
    fn test_sashimi_xwing() {
        let s = ":0320:3:......3+8+99.4..2+561....9.72+4+4619+2+78+53+8+5+93+64+17+2..2...+4+9+6.97.1..4+85....8+9.+7.....+9..+5::371:c36 r37 fr8c3 fr9c3";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = FishFinder::new(2, false, true);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }

    #[test]
    fn test_finned_swordfish() {
        let s = ":0311:7:+2.3.+186+5.41+6+75+39+8+2.+5+8.+26.1.84.3+6+2.9+5+62.+8.543.5+3.1+4.+8+2+6.+6+52...+4+83.+4+58.26..+8+2+6.45+7.::737:c159 r357 fr1c9";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = FishFinder::new(3, true, false);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
        let hint = steps.iter().next().unwrap();
        match hint {
            Step::Fish(fish) => {
                assert_eq!(fish.value, 7);
            }
            _ => {
                assert!(false);
            }
        }
    }
    #[test]
    fn test_sashimi_swordfish() {
        let s = ":0321:2:2.7+89+5+6.+15..7.+4+9.8.9+8..6......+4.+9......6.+8.938.9.5+3764...+3+62......54+7...+7.3+9+814.6::245 255:r269 c258 fr6c4";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = FishFinder::new(3, false, true);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 2);
        let hint = steps.iter().next().unwrap();
        match hint {
            Step::Fish(fish) => {
                assert_eq!(fish.value, 2);
            }
            _ => {
                assert!(false);
            }
        }
    }
    #[test]
    fn test_finned_jellfish() {
        let s = "...16.87..1.875..38.73..651.5.62173...17..5.473.5..1...7........8.256917.62..7...";
        let grid = Grid::new_from_singline_digit(s).unwrap();
        let finder = FishFinder::new(4, true, false);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }
    #[test]
    fn test_sashimi_jellfish() {
        let s = "..34162..26...31.41.4....36.463715.2.2184......762.41...5.3..41..21.4...41.56732.";
        let grid = Grid::new_from_singline_digit(s).unwrap();
        let finder = FishFinder::new(4, false, true);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 2);
    }
    #[test]
    fn test_basic_xwing() {
        let s = ":0300:5:.+4+1+7+2+9.+3.76+9..3+4.2.+3264.+7+194.39..+17.+6.+7..49.3+1+95+3+7..2+4+21+456+7+3+9+837+6.9.+541+9+5+8+4+3+1+26+7::545:r25 c58";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = FishFinder::new(2, false, false);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 1);
    }

    #[test]
    fn test_sword_fish() {
        let s = ":0301:2:16.54+3.7..+78+6.1+43+5+43+58.+7+6.+17+2.+45+8.696..9+12.57...+3+7+6..+4.+1+6.3..4.+3...+8..16..+71645.+3::268 271:r239 c158";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = FishFinder::new(3, false, false);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 2);
    }

    #[test]
    fn test_jell_fish() {
        let s = ":0302:7:2.......3.8..3..5...34.21....12.54......9......93.86....25.69...9..2..7.4.......1::712 715 721 729 751 752 759 792 795:r3467 c1259";
        let grid = Grid::new_from_hodoku_line(s).unwrap();
        let finder = FishFinder::new(4, false, false);
        let mut acc = AllStepAccumulator::default();
        finder.find_step(&grid, &mut acc);
        let steps = acc.get_steps();
        assert_eq!(steps.len(), 2);
    }
}
