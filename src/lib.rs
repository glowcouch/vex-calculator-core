use spacers::{VexSpacer, VexSpacerSolution};
use uom::si::f32::*;

pub mod spacers;

#[derive(Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct VexLength {
    pub length: Length,
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct VexSpacerSolveCenario {
    pub max_washers: u32,
}

impl VexLength {
    pub fn calculate_spacers(&self, cenario: VexSpacerSolveCenario) -> Vec<VexSpacerSolution> {
        let mut solutions: Vec<VexSpacerSolution> = vec![];
        let spacers = spacers::VexSpacer::get_spacers();

        fn recurse(
            length: &Length,
            spacers: &Vec<VexSpacer>,
            cenario: &VexSpacerSolveCenario,
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
                    if !(current_solution.get_washers() >= cenario.max_washers
                        && spacer.kind.is_washer())
                    {
                        let mut current_solution = current_solution.clone();
                        current_solution.spacers.push(spacer.clone());
                        recurse(
                            length,
                            spacers,
                            &cenario,
                            current_solution.clone(),
                            solutions,
                        );
                    }
                }
            }
        }

        recurse(
            &self.length,
            &spacers,
            &cenario,
            VexSpacerSolution::default(),
            &mut solutions,
        );

        solutions
    }
}
