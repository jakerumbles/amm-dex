#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::PalletId;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

mod traits;
pub mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use codec::{Codec, FullCodec};
use frame_support::{dispatch::EncodeLike, pallet_prelude::*, Blake2_128Concat};
use frame_system::pallet_prelude::*;
use orml_traits::currency::{MultiCurrency, TransferAll};
use orml_traits::{
	MultiCurrencyExtended, MultiLockableCurrency, MultiReservableCurrency,
	NamedMultiReservableCurrency,
};
use sp_runtime::traits::{AtLeast32BitUnsigned, Zero};
use sp_std::fmt::Debug;
use types::*;

// VAULT ADDRESS (random hopefully it works)
const VAULT_ADDRESS: PalletId = PalletId(*b"5CiPPseX");

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	pub(crate) type CurrencyIdOf<T> =
		<<T as pallet::Config>::Tokens as orml_traits::MultiCurrency<
			<T as frame_system::Config>::AccountId,
		>>::CurrencyId;

	pub(crate) type BalanceOf<T> = <<T as pallet::Config>::Tokens as orml_traits::MultiCurrency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currencies: MultiCurrency<Self::AccountId>
			+ MultiCurrencyExtended<Self::AccountId>
			+ MultiLockableCurrency<Self::AccountId>
			+ MultiReservableCurrency<Self::AccountId>
			+ NamedMultiReservableCurrency<Self::AccountId>;

		type Tokens: TransferAll<Self::AccountId>
			+ MultiCurrencyExtended<Self::AccountId>
			+ MultiLockableCurrency<Self::AccountId>
			+ MultiReservableCurrency<Self::AccountId>
			+ NamedMultiReservableCurrency<Self::AccountId>;

		type Balance: Parameter
			+ Member
			+ AtLeast32BitUnsigned
			+ Codec
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ MaxEncodedLen
			+ TypeInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn pairs)]
	pub type Pairs<T: Config> =
		StorageMap<_, Blake2_128Concat, (CurrencyIdOf<T>, CurrencyIdOf<T>), Pair<T>>;

	#[pallet::storage]
	#[pallet::getter(fn liquidity_mapping_a)]
	pub type LiquidityMappingA<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(CurrencyIdOf<T>, CurrencyIdOf<T>),
		CurrencyIdOf<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn liquidity_mapping_b)]
	pub type LiquidityMappingB<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		CurrencyIdOf<T>,
		(CurrencyIdOf<T>, CurrencyIdOf<T>),
		OptionQuery,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [token0, token1, who_created]
		PairCreated(CurrencyIdOf<T>, CurrencyIdOf<T>, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Trying to create a pair with at least one non-existant token
		InvalidToken,
		/// Pair tokens cannot be the same
		SameTokens,
		/// Pair already exists
		PairAlreadyExists,
		/// Invalid amount, must be greater than 0
		InvalidAmount,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new pool and seed with some initial liquidity
		#[pallet::weight(1)]
		pub fn create_pair(
			origin: OriginFor<T>,
			token_0: CurrencyIdOf<T>,
			token_0_amount: BalanceOf<T>,
			token_1: CurrencyIdOf<T>,
			token_1_amount: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Perform checks before calling internal `create_pair` function
			ensure!(token_0 != token_1, Error::<T>::SameTokens);
			ensure!(Pairs::<T>::contains_key((token_0, token_1)), Error::<T>::PairAlreadyExists);
			ensure!(Pairs::<T>::contains_key((token_1, token_0)), Error::<T>::PairAlreadyExists);
			ensure!(
				!token_0_amount.is_zero() && !token_1_amount.is_zero(),
				Error::<T>::InvalidAmount
			);

			// Verify caller (`who`) has enough tokens
			T::Tokens::ensure_can_withdraw(token_0, &who, token_0_amount)?;
			T::Tokens::ensure_can_withdraw(token_1, &who, token_1_amount)?;

			<Self as XykAmm<T>>::create_pair(
				who,
				token_0,
				token_0_amount,
				token_1,
				token_1_amount,
			)?;

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {}

	pub trait XykAmm<T: Config> {
		fn create_pair(
			who: T::AccountId,
			token_0: CurrencyIdOf<T>,
			token_0_amount: BalanceOf<T>,
			token_1: CurrencyIdOf<T>,
			token_1_amount: BalanceOf<T>,
		) -> DispatchResult;
	}

	impl<T: Config> XykAmm<T> for Pallet<T> {
		fn create_pair(
			who: T::AccountId,
			token_0: CurrencyIdOf<T>,
			token_0_amount: BalanceOf<T>,
			token_1: CurrencyIdOf<T>,
			token_1_amount: BalanceOf<T>,
		) -> DispatchResult {
			// Create `Pair`
			let pair = Pair::<T>::new(token_0, token_0_amount, token_1, token_1_amount);

			// Insert `pair` to storage
			Pairs::<T>::insert((token_0, token_1), pair);

			// Calculate LP tokens to create
			let liquidity = token_0_amount + token_1_amount;

			// Mint LP tokens `mint_into` line 1687

			// Send LP tokens to `who`

			// Update storage mapping (AccountId, CurrencyIdOf) => BalanceOf

			Ok(())
		}
	}
}
