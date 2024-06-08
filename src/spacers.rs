use std::fmt;
use uom::si::f32::*;
use uom::si::length::{inch, millimeter};

#[derive(Clone, PartialEq)]
pub struct VexSpacer {
    pub thickness: Length,
    pub od: Length,
    pub kind: String,
}

impl fmt::Display for VexSpacer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(thickness: {} mm, od: {} mm)",
            self.thickness.value * 1000.0,
            self.od.value * 1000.0
        )
    }
}

impl VexSpacer {
    pub fn get_spacers() -> Vec<VexSpacer> {
        vec![
            // 8mm plastic spacers
            VexSpacer {
                thickness: Length::new::<millimeter>(8.0),
                od: Length::new::<inch>(0.32),
                kind: "8mm black plastic spacer".to_string(),
            },
            // 3/8" OD nylon spacers
            /*VexSpacer {
                thickness: Length::new::<inch>(0.125),
                od: Length::new::<inch>(3.0 / 8.0),
                kind: "nylon spacer".to_string(),
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.25),
                od: Length::new::<inch>(3.0 / 8.0),
                kind: "nylon spacer".to_string(),
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.375),
                od: Length::new::<inch>(3.0 / 8.0),
                kind: "nylon spacer".to_string(),
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.5),
                od: Length::new::<inch>(3.0 / 8.0),
                kind: "nylon spacer".to_string(),
            },*/
            // 1/2" OD nylon spacers
            VexSpacer {
                thickness: Length::new::<inch>(0.125),
                od: Length::new::<inch>(1.0/2.0),
                kind: "nylon spacer".to_string(),
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.25),
                od: Length::new::<inch>(1.0/2.0),
                kind: "nylon spacer".to_string(),
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.375),
                od: Length::new::<inch>(1.0/2.0),
                kind: "nylon spacer".to_string(),
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.5),
                od: Length::new::<inch>(1.0/2.0),
                kind: "nylon spacer".to_string(),
            },
            // Teflon washer
            VexSpacer {
                thickness: Length::new::<inch>(0.04),
                od: Length::new::<inch>(0.375),
                kind: "teflon washer".to_string(),
            },
            // Steel washer
            VexSpacer {
                thickness: Length::new::<inch>(0.032),
                od: Length::new::<inch>(0.375),
                kind: "steel washer".to_string(),
            },
        ]
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct VexSpacerSolution {
    pub spacers: Vec<VexSpacer>,
    pub target: Length,
}

impl VexSpacerSolution {
    pub fn get_thickness(&self) -> Length {
        let mut thickness: Length = Length::default();
        for spacer in &self.spacers {
            thickness += spacer.thickness
        }
        thickness
    }
    pub fn get_od(&self) -> Length {
        let mut od: Length = Length::default();
        for spacer in &self.spacers {
            if spacer.od > od {
                od = spacer.od;
            }
        }
        od
    }
    pub fn get_error(&self) -> Length {
        self.get_thickness() - self.target
    }
    pub fn is_perfect(&self) -> bool {
        self.get_thickness() == self.target
    }
}
