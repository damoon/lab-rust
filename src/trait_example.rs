#[cfg(test)]
mod tests {
    use super::Some;
    use super::Test;
    use super::Thing;

    #[test]
    fn use_trait() {
        let mut aa: Vec<Box<dyn Test>> = Vec::new();
        aa.push(Box::new(Thing {}));
        aa.push(Box::new(Some {}));
        for a in aa.into_iter() {
            super::abc0(a.as_ref());
            super::abc1(a);
        }
    }
}

pub trait Test {
    fn def(&self) -> String;
}

struct Thing {}

struct Some {}

impl Test for Thing {
    fn def(&self) -> String {
        "a".to_string()
    }
}

impl Test for Some {
    fn def(&self) -> String {
        "a".to_string()
    }
}

pub fn abc0(bla: &dyn Test) {
    println!("v: {:?}", bla.def());
}

pub fn abc1(bla: Box<dyn Test>) {
    println!("v: {:?}", bla.def());
}
