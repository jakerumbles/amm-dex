use crate::Config;
use codec::{Decode, Encode, EncodeLike, MaxEncodedLen};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, PartialEq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Pair<T: Config> {
	pub token_0: T::CurrencyId,
	pub token_1: T::CurrencyId,
	token_0_amount: T::Balance,
	token_1_amount: T::Balance,
}
