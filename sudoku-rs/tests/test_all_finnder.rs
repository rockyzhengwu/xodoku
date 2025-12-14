use sudoku_rs::{
    grid::Grid,
    solver::{
        SolverStrategy, avoidable_rectangle_1, avoidable_rectangle_2, bug_plus_one, chain,
        empty_rectangle, fish, full_house, hidden_rectangle, hidden_set, hidden_single,
        locked_candidate, naked_set, naked_single, skyscraper, step::Step,
        step_accumulator::SingleStepAccumulator, sue_de_coq, two_string_kit, unique_1, unique_2,
        unique_3, unique_4, unique_5, unique_6, wwing, xywing,
    },
};

#[test]
fn test_all_finder() {
    let s = "...481.5.3......9.1...7...47....3.6...65....3....9...8....3...2....57....7....8.9";
    let s = ":0800:2:+8..+36.+9....+9.1.863.+63.+89..+59+24+6+7+3+1+5+83+8+6+9+5+17+2457+182+4+3+9+6+4+3+2+1+9658+769+8+5+37......+24+8+63+9::226:";
    let expected_solution =
        "857362941249715863163489275924673158386951724571824396432196587698537412715248639";
    let solution: Vec<u8> = expected_solution
        .chars()
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let strategies: Vec<Box<dyn SolverStrategy>> = vec![
        Box::new(full_house::FullHouseFinder::default()),
        Box::new(naked_single::NakedSingleFinder::default()),
        Box::new(hidden_single::HiddenSingleFinder::default()),
        Box::new(naked_set::NakedSetFinder::new(2)),
        Box::new(naked_set::NakedSetFinder::new(3)),
        Box::new(hidden_set::HiddenSetFinder::new(2)),
        Box::new(hidden_set::HiddenSetFinder::new(3)),
        Box::new(locked_candidate::LockedCandidateFinder::new(
            locked_candidate::LockedCandidateType::Pointing,
        )),
        Box::new(locked_candidate::LockedCandidateFinder::new(
            locked_candidate::LockedCandidateType::Claiming,
        )),
        Box::new(avoidable_rectangle_1::AvoidableRectangleType1Finder::default()),
        Box::new(avoidable_rectangle_2::AvoidableRectangleType2Finder::default()),
        Box::new(unique_1::Unique1Finder::default()),
        Box::new(unique_2::Unique2Finder::default()),
        Box::new(unique_3::Unique3Finder::default()),
        Box::new(unique_4::Unique4Finder::default()),
        Box::new(unique_5::Unique5Finder::default()),
        Box::new(unique_6::Unique6Finder::default()),
        Box::new(empty_rectangle::EmptyRectangleFinder::default()),
        Box::new(hidden_rectangle::HiddenRectangleFinder::default()),
        Box::new(bug_plus_one::BugPlusOneFinder::default()),
        Box::new(skyscraper::SkyscraperFinder::default()),
        Box::new(sue_de_coq::SueDeCoqFinder::default()),
        Box::new(fish::FishFinder::new(fish::FishType::XWing)),
        Box::new(chain::remote_pair::RemotePairFinder::default()),
        Box::new(two_string_kit::TwoStringKitFinder::default()),
        Box::new(fish::FishFinder::new(fish::FishType::SwordFish)),
        Box::new(fish::FishFinder::new(fish::FishType::JellyFish)),
        Box::new(xywing::XYWingFinder::default()),
        Box::new(wwing::WWingFinder::default()),
        Box::new(fish::FishFinder::new(fish::FishType::FinnedXWing)),
        Box::new(fish::FishFinder::new(fish::FishType::SashimiXWing)),
        Box::new(fish::FishFinder::new(fish::FishType::FinnedSowrdFish)),
        Box::new(fish::FishFinder::new(fish::FishType::SashimiSwordFish)),
        Box::new(fish::FishFinder::new(fish::FishType::FinnedJellFish)),
        Box::new(fish::FishFinder::new(fish::FishType::SashimiJellyFish)),
        Box::new(chain::x_chain::XChainFinder::default()),
        Box::new(chain::xy_chain::XYChainFinder::default()),
        Box::new(chain::discontinuous_nice_loop::DiscontinuousNiceLoopFinder::default()),
        Box::new(chain::continuous_nice_loop::ContinuousNiceLoopFinder::default()),
        Box::new(chain::aic_type1::AicType1Finder::default()),
        Box::new(chain::aic_type2::AicType2Finder::default()),
    ];
    let mut grid = Grid::new_from_hodoku_line(s).unwrap();
    loop {
        if grid.is_solved() {
            break;
        }
        let mut changed = false;
        for finder in strategies.iter() {
            let mut acc = SingleStepAccumulator::default();
            finder.find_step(&grid, &mut acc);
            let step = acc.get_step();
            match step {
                Step::Nothing => {
                    continue;
                }
                _ => {
                    step.apply(&mut grid);
                    changed = true;
                    if !grid.check_grid_valid(solution.as_slice()) {
                        println!("invalid step:{:?}", step);
                        println!("{}", grid.to_digit_line());
                        return;
                    }
                    break;
                }
            }
        }
        if !changed {
            println!("no step");
            break;
        }
    }
    println!("{:?}", grid.is_solved());
    println!("{:?}", grid.to_digit_line());
    assert_eq!(grid.is_solved(), true);
    assert_eq!(grid.to_digit_line(), expected_solution);
}
