use spacers::{VexSpacer, VexSpacerSolution};
use uom::si::f32::*;
use uom::si::length::inch;

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
            let mut spacers_length: Length = Length::default();

            for spacer in current_solution.spacers.clone() {
                spacers_length += spacer.thickness;
            }

            if &spacers_length == length {
                let mut current_solution = current_solution.clone();
                current_solution.perfect = true;
                current_solution.target = *length;
                solutions.push(current_solution.clone());
            } else if &spacers_length > length {
                let mut current_solution = current_solution.clone();
                current_solution.perfect = false;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = VexLength {
            length: Length::new::<inch>(0.25),
        }
        .calculate_spacers();
        assert!(result.contains(&VexSpacerSolution {
            spacers: vec![VexSpacer {
                thickness: Length::new::<inch>(0.25),
                od: Length::new::<inch>(3.0 / 8.0),
                kind: "nylon spacer".to_string(),
            },],
            perfect: true,
            target: Length::new::<inch>(0.25),
        }));
    }
}
