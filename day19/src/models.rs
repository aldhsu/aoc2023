pub struct Rule {
    pub name: String,
    pub conditions: Vec<CondType>,
}

#[derive(Clone, Debug)]
pub struct RangeSet(isize, isize);

impl RangeSet {
    pub fn count(&self) -> isize {
        if self.0 >= self.1 {
            return 0;
        };
        isize::abs(self.1 - self.0) + 1
    }

    pub fn to_ranges(operator: &Operator, target: &isize) -> (Self, Self) {
        match operator {
            Operator::Greater => (RangeSet(target + 1, COND_MAX), RangeSet(1, *target)),
            Operator::Lesser => (RangeSet(1, target - 1), RangeSet(*target, COND_MAX)),
        }
    }
}

const COND_MAX: isize = 4000;
impl Rule {
    pub fn apply(&self, xmas: &Xmas) -> &Outcome {
        for cond in &self.conditions {
            if let Some(val) = cond.apply(xmas) {
                return val;
            }
        }
        unreachable!()
    }
}

pub enum Operator {
    Greater,
    Lesser,
}

#[derive(Eq, PartialEq, Hash)]
pub enum FieldName {
    X,
    M,
    A,
    S,
}

pub enum CondType {
    Unconditional(Outcome),
    Cond {
        operator: Operator,
        comparator: isize,
        field_name: FieldName,
        target: Outcome,
    },
}

impl CondType {
    pub fn apply(&self, xmas: &Xmas) -> Option<&Outcome> {
        match self {
            CondType::Unconditional(outcome) => Some(outcome),
            CondType::Cond {
                operator,
                comparator,
                field_name,
                target,
            } => {
                let is_applicable: bool = {
                    let left = match field_name {
                        FieldName::X => xmas.x,
                        FieldName::M => xmas.m,
                        FieldName::A => xmas.a,
                        FieldName::S => xmas.s,
                    };

                    match operator {
                        Operator::Greater => &left > comparator,
                        Operator::Lesser => &left < comparator,
                    }
                };

                if !is_applicable {
                    return None;
                }
                Some(target)
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub enum Outcome {
    Target(String),
    #[default]
    Accepted,
    Rejected,
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Xmas {
    pub x: isize,
    pub m: isize,
    pub a: isize,
    pub s: isize,
}

impl Xmas {
    pub fn total(&self) -> isize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Default, Clone, Debug)]
pub struct Permutation {
    x: Option<RangeSet>,
    m: Option<RangeSet>,
    a: Option<RangeSet>,
    s: Option<RangeSet>,
}

impl Permutation {
    pub fn combos(&self) -> isize {
        fn count_range(range: &Option<RangeSet>) -> isize {
            match range {
                Some(range) => range.count(),
                None => COND_MAX,
            }
        }

        let xs = count_range(&self.x);
        let ms = count_range(&self.m);
        let a_s = count_range(&self.a);
        let ss = count_range(&self.s);

        xs * ms * a_s * ss
    }

    pub fn add(&mut self, field_name: &FieldName, range: RangeSet) {
        fn update_field(field: &mut Option<RangeSet>, RangeSet(b_low, b_high): RangeSet) {
            match field {
                Some(RangeSet(a_low, a_high)) => {
                    let new_low = (*a_low).max(b_low);
                    let new_high = (*a_high).min(b_high);
                    *field = Some(RangeSet(new_low, new_high));
                }
                None => {
                    *field = Some(RangeSet(b_low, b_high));
                }
            }
        }
        let field = match field_name {
            FieldName::X => &mut self.x,
            FieldName::M => &mut self.m,
            FieldName::A => &mut self.a,
            FieldName::S => &mut self.s,
        };
        update_field(field, range);
    }
}

#[test]
fn range_add_test() {
    let mut r = Permutation {
        x: None,
        m: None,
        a: None,
        s: None,
    };
    assert_eq!(r.combos(), 0);
}
