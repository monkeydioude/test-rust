pub trait IO<Type>{
    fn get_data(&self) -> Type;
}

pub trait Unit<InType> {
    fn process(&self, t: dyn IO<InType>);
}

struct UnitA;

impl Unit<f64> for UnitA {
    fn process(&self, t: dyn IO<f64>) {
        todo!()
    }
}



pub fn main() {
    let units: Vec<&dyn Unit<&dyn , &dyn IO>> = vec![
        // &UnitA::<f64>::new(),
        // &UnitB::new(),
    ];
}