use std::fmt;
use uom::si::f32::*;
use uom::si::length::{inch, millimeter};

#[derive(Clone, Copy, PartialEq)]
pub struct VexSpacer {
    pub thickness: Length,
    pub od: Length,
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
            },
            // 3/8" OD nylon spacers
            VexSpacer {
                thickness: Length::new::<inch>(0.125),
                od: Length::new::<inch>(3.0 / 8.0),
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.25),
                od: Length::new::<inch>(3.0 / 8.0),
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.375),
                od: Length::new::<inch>(3.0 / 8.0),
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.5),
                od: Length::new::<inch>(3.0 / 8.0),
            },
            // 1/2" OD nylon spacers
            /*VexSpacer {
                thickness: Length::new::<inch>(0.125),
                od: Length::new::<inch>(1.0/2.0),
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.25),
                od: Length::new::<inch>(1.0/2.0),
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.375),
                od: Length::new::<inch>(1.0/2.0),
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.5),
                od: Length::new::<inch>(1.0/2.0),
            },*/
            // Teflon washer
            VexSpacer {
                thickness: Length::new::<inch>(0.04),
                od: Length::new::<inch>(0.375),
            },
            // Steel washer
            VexSpacer {
                thickness: Length::new::<inch>(0.032),
                od: Length::new::<inch>(0.375),
            },
        ]
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct VexSpacerSolution {
    pub spacers: Vec<VexSpacer>,
    pub perfect: bool,
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
}
