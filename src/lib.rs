use spacers::{VexSpacer, VexSpacerSolution};
use uom::si::f32::*;

pub mod spacers;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct VexLength {
    pub length: Length,
}

impl VexLength {
    pub fn calculate_spacers(&self) -> Vec<VexSpacerSolution> {
        let mut solutions: Vec<VexSpacerSolution> = vec![];
        let spacers = spacers::VexSpacer::get_spacers();

        fn recurse(
            length: &Length,
            spacers: &Vec<VexSpacer>,
            current_solution: VexSpacerSolution,
            solutions: &mut Vec<VexSpacerSolution>,
        ) {
            let spacers_length: Length = current_solution.get_thickness();

            if &spacers_length == length {
                let mut current_solution = current_solution;
                current_solution.target = *length;
                solutions.push(current_solution);
            } else if &spacers_length > length {
                let mut current_solution = current_solution;
                current_solution.target = *length;
                solutions.push(current_solution.clone());
                current_solution
                    .spacers
                    .remove(current_solution.spacers.len() - 1);
                solutions.push(current_solution);
            } else {
                for spacer in spacers {
                    let mut current_solution = current_solution.clone();
                    current_solution.spacers.push(spacer.clone());
                    recurse(length, spacers, current_solution.clone(), solutions);
                }
            }
        }

        recurse(
            &self.length,
            &spacers,
            VexSpacerSolution::default(),
            &mut solutions,
        );

        solutions
    }
}
