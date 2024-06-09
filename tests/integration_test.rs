use vex_calculator_core::{spacers::*, VexLength};

#[test]
fn it_finds_single_spacer_solutions() {
    for spacer in VexSpacer::get_spacers() {
        let result = VexLength { length: spacer.thickness }.calculate_spacers();
        assert!(result.contains(&VexSpacerSolution {
            spacers: vec![spacer.clone()],
            target: spacer.thickness,
        }));
    }
}
