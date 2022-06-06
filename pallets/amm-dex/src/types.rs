use crate::Config;
use crate::{BalanceOf, CurrencyIdOf, Moment};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::Time;
use scale_info::TypeInfo;
use sp_runtime::traits::CheckedMul;
use sp_std::convert;

#[derive(Clone, Encode, Decode, PartialEq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Pair<T: Config> {
	pub token_0: CurrencyIdOf<T>,
	/// Balance of `token_0` for pair
	pub reserve_0: BalanceOf<T>,
	pub token_1: CurrencyIdOf<T>,
	/// Balance of `token_1` for pair
	pub reserve_1: BalanceOf<T>,
	/// `reserve_0` * `reserve_1`. Changes when a liquidity provider deposits or withdraws tokens, and it increases slightly because of the 0.3% market fee.
	pub k_last: BalanceOf<T>,
	pub block_timestamp_last: Moment<T>,
	pub price_0_cumulative_last: BalanceOf<T>,
	pub price_1_cumulative_last: BalanceOf<T>,
}

impl<T: Config> Pair<T> {
	pub fn new(
		token_0: CurrencyIdOf<T>,
		reserve_0: BalanceOf<T>,
		token_1: CurrencyIdOf<T>,
		reserve_1: BalanceOf<T>,
	) -> Self {
		let k_last = reserve_0.checked_mul(&reserve_1).expect("Overflow. k_last too big!");
		let block_timestamp_last = T::Time::now();
		let price_0_cumulative_last = 0;
		let price_1_cumulative_last = 0;

		Self {
			token_0,
			reserve_0,
			token_1,
			reserve_1,
			k_last,
			block_timestamp_last,
			price_0_cumulative_last,
			price_1_cumulative_last,
		}
	}
}
