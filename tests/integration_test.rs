use vex_calculator_core::{spacers::*, VexLength, VexSpacerSolveCenario};

#[test]
fn it_finds_single_spacer_solutions() {
    for spacer in VexSpacer::get_spacers() {
        let length = VexLength { length: spacer.thickness };
        let result = length.calculate_spacers(
            VexSpacerSolveCenario {
                max_washers: 1,
            });
        assert!(result.contains(&VexSpacerSolution {
            spacers: vec![spacer.clone()],
            target: spacer.thickness,
        }));
    }
}
