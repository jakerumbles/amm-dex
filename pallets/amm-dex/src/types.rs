use crate::Config;
use crate::{BalanceOf, CurrencyIdOf};
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, PartialEq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Pair<T: Config> {
	pub token_0: CurrencyIdOf<T>,
	pub reserve_0: BalanceOf<T>,
	pub token_1: CurrencyIdOf<T>,
	pub reserve_1: BalanceOf<T>,
}

impl<T: Config> Pair<T> {
	pub fn new(
		token_0: CurrencyIdOf<T>,
		reserve_0: BalanceOf<T>,
		token_1: CurrencyIdOf<T>,
		reserve_1: BalanceOf<T>,
	) -> Self {
		Self { token_0, reserve_0, token_1, reserve_1 }
	}
}
