pub trait Builder<F, const N: usize>
where F: Fn(f64) -> f64 {
	fn build(&self) -> ClosuresHolder<F, N>;
}
struct HolderBuilder<F, const N: usize>
where F: Fn(f64) -> f64 {
	pub cs: Vec<F>
}

pub struct ClosuresHolder<F, const N: usize>
where F: Fn(f64) -> f64 {
	cs: Vec<F>
}

impl<F, const N: usize> HolderBuilder<F, N>
where F: Fn(f64) -> f64 + Clone {
	pub fn new(cs: Vec<F>) -> Self {
		HolderBuilder { cs }
	}
}

impl<F, const N: usize> Builder<F, N> for HolderBuilder<F, N>
where F: Fn(f64) -> f64 + Clone {
	fn build(&self) -> ClosuresHolder<F, N> {
		ClosuresHolder { cs: self.cs.clone() }
	}
}

pub fn main() {
	let hb = HolderBuilder::<_ ,1>::new(
		vec![
			|v: f64| -> f64 {
				v * 2.
			}
		]
	);
	
	let ch = hb.build();
	ch.cs.iter().for_each(|v| {
		println!("{}", v(2.0));
	})
}
