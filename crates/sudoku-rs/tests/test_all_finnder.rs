use sudoku_rs::{
    grid::Grid,
    solver::{
        SolverStrategy, als, chain, coloring, complex_fish, empty_rectangle, fish, full_house,
        hidden_set, hidden_single, locked_candidate, naked_set, naked_single, skyscraper,
        step::Step, step_accumulator::SingleStepAccumulator, sue_de_coq, turbot_fish,
        two_string_kit, unique, wings,
    },
};

#[test]
fn test_all_finder() {
    let _s = "...481.5.3......9.1...7...47....3.6...65....3....9...8....3...2....57....7....8.9";
    let s = ":0800:2:+8..+36.+9....+9.1.863.+63.+89..+59+24+6+7+3+1+5+83+8+6+9+5+17+2457+182+4+3+9+6+4+3+2+1+9658+769+8+5+37......+24+8+63+9::226:";
    let expected_solution =
        "857362941249715863163489275924673158386951724571824396432196587698537412715248639";
    let solution: Vec<u8> = expected_solution
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let strategies: Vec<Box<dyn SolverStrategy>> = vec![
        Box::new(full_house::FullHouseFinder::default()),
        Box::new(naked_single::NakedSingleFinder::default()),
        Box::new(hidden_single::HiddenSingleFinder::default()),
        Box::new(naked_set::NakedSetFinder::new(2)),
        Box::new(naked_set::NakedSetFinder::new(3)),
        Box::new(naked_set::NakedSetFinder::new(4)),
        Box::new(hidden_set::HiddenSetFinder::new(2)),
        Box::new(hidden_set::HiddenSetFinder::new(3)),
        Box::new(hidden_set::HiddenSetFinder::new(4)),
        Box::new(locked_candidate::LockedCandidateFinder::new(
            locked_candidate::LockedCandidateType::Pointing,
        )),
        Box::new(locked_candidate::LockedCandidateFinder::new(
            locked_candidate::LockedCandidateType::Claiming,
        )),
        Box::new(unique::UniquenessFinder::default()),
        Box::new(unique::MissingCandidatesUniquenessFinder),
        Box::new(sue_de_coq::SueDeCoqFinder::default()),
        Box::new(sue_de_coq::ExtendedSueDeCoqFinder::default()),
        Box::new(fish::FishFinder::new(fish::FishType::basic(
            fish::FishSize::XWing,
        ))),
        Box::new(chain::remote_pair::RemotePairFinder::default()),
        Box::new(two_string_kit::TwoStringKitFinder::default()),
        Box::new(skyscraper::SkyscraperFinder::default()),
        Box::new(turbot_fish::TurbotFishFinder::default()),
        Box::new(empty_rectangle::EmptyRectangleFinder::default()),
        Box::new(fish::FishFinder::new(fish::FishType::basic(
            fish::FishSize::Swordfish,
        ))),
        Box::new(fish::FishFinder::new(fish::FishType::basic(
            fish::FishSize::Jellyfish,
        ))),
        Box::new(wings::WingsFinder::default()),
        Box::new(fish::FishFinder::new(fish::FishType::finned(
            fish::FishSize::XWing,
        ))),
        Box::new(fish::FishFinder::new(fish::FishType::sashimi(
            fish::FishSize::XWing,
        ))),
        Box::new(fish::FishFinder::new(fish::FishType::basic(
            fish::FishSize::Squirmbag,
        ))),
        Box::new(fish::FishFinder::new(fish::FishType::basic(
            fish::FishSize::Whale,
        ))),
        Box::new(fish::FishFinder::new(fish::FishType::basic(
            fish::FishSize::Leviathan,
        ))),
        Box::new(fish::FishFinder::new(fish::FishType::finned(
            fish::FishSize::Swordfish,
        ))),
        Box::new(fish::FishFinder::new(fish::FishType::sashimi(
            fish::FishSize::Swordfish,
        ))),
        Box::new(fish::FishFinder::new(fish::FishType::finned(
            fish::FishSize::Jellyfish,
        ))),
        Box::new(fish::FishFinder::new(fish::FishType::sashimi(
            fish::FishSize::Jellyfish,
        ))),
        Box::new(chain::x_chain::XChainFinder::default()),
        Box::new(coloring::ColoringFinder),
        Box::new(chain::xy_chain::XYChainFinder::default()),
        Box::new(chain::advanced::AicFinder),
        Box::new(als::AlsXzFinder),
        Box::new(als::AlsXYWingFinder),
        Box::new(als::AlsChainFinder),
        Box::new(chain::advanced::GroupedAicFinder),
        Box::new(als::AlsAicFinder),
        Box::new(als::DeathBlossomFinder),
        Box::new(complex_fish::ComplexFishFinder),
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
    assert!(grid.is_solved());
    assert_eq!(grid.to_digit_line(), expected_solution);
}
