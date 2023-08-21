

#[derive(Debug, Clone)]
pub struct Box{
	pub weight: u32,
	pub id: String,
}


#[derive(Debug, Clone)]
pub struct Tote {
	pub number_of_itesm: u32,
	pub total_weight: u32,
	pub id: String,
}

pub trait PullData {
	fn pull_data(&self) -> &Self {
		self
	}
}

impl PullData for Box {
	fn pull_data(&self) -> &Self{
		self
	}
}

impl PullData for Tote{
	fn pull_data(&self) -> &Self{
		self
	}
}