use crate::Config;
use crate::{BalanceOf, CurrencyIdOf};
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, PartialEq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Pair<T: Config> {
	pub token_0: CurrencyIdOf<T>,
	pub token_0_amount: BalanceOf<T>,
	pub token_1: CurrencyIdOf<T>,
	pub token_1_amount: BalanceOf<T>,
}

impl<T: Config> Pair<T> {
	pub fn new(
		token_0: CurrencyIdOf<T>,
		token_0_amount: BalanceOf<T>,
		token_1: CurrencyIdOf<T>,
		token_1_amount: BalanceOf<T>,
	) -> Self {
		Self { token_0, token_0_amount, token_1, token_1_amount }
	}
}
