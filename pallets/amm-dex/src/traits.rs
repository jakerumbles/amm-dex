use crate::Config;

pub trait PairFactory<T: Config> {
	fn create_pair(&mut self, token_a: T::CurrencyId, token_b: T::CurrencyId) -> T::PairId;
}
