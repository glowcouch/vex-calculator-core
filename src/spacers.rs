use std::fmt;
use uom::si::f32::*;
use uom::si::length::{inch, millimeter};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VexSpacerKind {
    Mm8BlackPlasticSpacer,
    NylonSpacer,
    TeflonWasher,
    SteelWasher,
}

impl VexSpacerKind {
    pub fn get_name(&self) -> String {
        match self {
            Self::Mm8BlackPlasticSpacer => "8mm black plastic spacer".to_string(),
            Self::NylonSpacer => "nylon spacer".to_string(),
            Self::TeflonWasher => "teflon washer".to_string(),
            Self::SteelWasher => "steel washer".to_string(),
        }
    }
    pub fn is_washer(&self) -> bool {
        match self {
            Self::SteelWasher => true,
            Self::TeflonWasher => true,
            _other => false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct VexSpacer {
    pub thickness: Length,
    pub od: Length,
    pub kind: VexSpacerKind,
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
                kind: VexSpacerKind::Mm8BlackPlasticSpacer,
            },
            // 3/8" OD nylon spacers
            /*VexSpacer {
                thickness: Length::new::<inch>(0.125),
                od: Length::new::<inch>(3.0 / 8.0),
                kind: 1,
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.25),
                od: Length::new::<inch>(3.0 / 8.0),
                kind: 1,
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.375),
                od: Length::new::<inch>(3.0 / 8.0),
                kind: 1,
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.5),
                od: Length::new::<inch>(3.0 / 8.0),
                kind: 1,
            },*/
            // 1/2" OD nylon spacers
            VexSpacer {
                thickness: Length::new::<inch>(0.125),
                od: Length::new::<inch>(1.0/2.0),
                kind: VexSpacerKind::NylonSpacer,
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.25),
                od: Length::new::<inch>(1.0/2.0),
                kind: VexSpacerKind::NylonSpacer,
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.375),
                od: Length::new::<inch>(1.0/2.0),
                kind: VexSpacerKind::NylonSpacer,
            },
            VexSpacer {
                thickness: Length::new::<inch>(0.5),
                od: Length::new::<inch>(1.0/2.0),
                kind: VexSpacerKind::NylonSpacer,
            },
            // Teflon washer
            VexSpacer {
                thickness: Length::new::<inch>(0.04),
                od: Length::new::<inch>(0.375),
                kind: VexSpacerKind::TeflonWasher,
            },
            // Steel washer
            VexSpacer {
                thickness: Length::new::<inch>(0.032),
                od: Length::new::<inch>(0.375),
                kind: VexSpacerKind::SteelWasher,
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
        (self.get_thickness() - self.target).abs() < Length::new::<inch>(0.001)
    }
    pub fn sort(&mut self){
        self.spacers.sort_unstable_by(|a, b| (a.thickness, a.od, a.kind.get_name()).partial_cmp(&(b.thickness, b.od, b.kind.get_name())).unwrap());
    }
    pub fn get_washers(&self) -> u32 {
        let mut washers: u32 = 0;
        for spacer in &self.spacers {
            if spacer.kind.is_washer() {
                washers += 1;
            }
        }
        washers
    }
}
