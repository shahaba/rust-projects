#[derive(Debug)]
enum Age {
    New,
    OneWeekOld,
    ThreeWeeksOld,
    OneMonthOld,
    Dead
}

#[derive(Debug)]
pub struct Asparagus {
    count: u32,
    age: Age
}

impl Asparagus {
    pub fn new() -> Self {
        Asparagus {
            count: 1,
            age: Age::New
        }
    }

    pub fn grow(&mut self) {
        print!("Growing from {:?} ", self.age);
        self.age = match self.age {
            Age::New => Age::OneWeekOld,
            Age::OneWeekOld => Age::ThreeWeeksOld,
            Age::ThreeWeeksOld => Age::OneMonthOld,
            Age::OneMonthOld => Age::Dead,
            _ => Age::Dead
        };

        println!("to {:?}", self.age)
    }
}