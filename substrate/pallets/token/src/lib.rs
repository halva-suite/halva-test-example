#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{Parameter, decl_module, decl_event, decl_storage, decl_error, ensure, dispatch};
use sp_runtime::traits::{Member, AtLeast32Bit, Zero, StaticLookup, MaybeSerializeDeserialize};
use codec::{Codec};
use frame_system::{self as system, ensure_signed};

/// The module configuration trait.
pub trait Trait: frame_system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	/// The units in which we record balances.
	type Balance: Parameter + Member + AtLeast32Bit + Codec + Default + Copy + MaybeSerializeDeserialize;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;
		
		#[weight = 10_000]
		pub fn give_me(
			origin,
			#[compact] amount: T::Balance
		) -> dispatch::DispatchResult {
			let origin = ensure_signed(origin)?;
			let origin_account = origin.clone();
			let origin_balance = <Balances<T>>::get(&origin_account);

			ensure!(origin_balance.is_zero() , "AccountId can request tokens only once");

			<Balances<T>>::insert(&origin_account, amount);
			<TotalSupply<T>>::mutate(|total_supply| *total_supply += amount);

			Ok(())
		}

		#[weight = 10_000]
		fn transfer(origin,
			target: <T::Lookup as StaticLookup>::Source,
			#[compact] amount: T::Balance
		) {
			let origin = ensure_signed(origin)?;
			let origin_account = origin.clone();
			let origin_balance = <Balances<T>>::get(&origin_account);
			let target = T::Lookup::lookup(target)?;
			ensure!(!amount.is_zero(), Error::<T>::AmountZero);
			ensure!(origin_balance >= amount, Error::<T>::BalanceLow);

			Self::deposit_event(RawEvent::Transferred(origin, target.clone(), amount));
			<Balances<T>>::insert(origin_account, origin_balance - amount);
			<Balances<T>>::mutate(target, |balance| *balance += amount);
		}
	}
}

decl_event! {
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
		<T as Trait>::Balance,
	{
		/// Some assets were transferred.
		Transferred(AccountId, AccountId, Balance),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Transfer amount should be non-zero
		AmountZero,
		/// Account balance must be greater than or equal to the transfer amount
		BalanceLow,
		/// Balance should be non-zero
		BalanceZero,
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as Tokens {
		/// The number of units of assets held by any given account.
		Balances: map hasher(blake2_128_concat) T::AccountId => T::Balance;
		/// The total unit supply of an asset.
		TotalSupply: T::Balance;
	}
}

// The main implementation block for the module.
impl<T: Trait> Module<T> {
	// Public immutables

	/// Get the asset `id` balance of `who`.
	pub fn balance(who: T::AccountId) -> T::Balance {
		<Balances<T>>::get(who)
	}

	/// Get the total supply of an asset `id`.
	pub fn total_supply() -> T::Balance {
		<TotalSupply<T>>::get()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use frame_support::{impl_outer_origin, assert_ok, assert_noop, parameter_types, weights::Weight};
	use sp_core::H256;
	// The testing primitives are very useful for avoiding having to work with signatures
	// or public keys. `u64` is used as the `AccountId` and no `Signature`s are required.
	use sp_runtime::{Perbill, traits::{BlakeTwo256, IdentityLookup}, testing::Header};

	impl_outer_origin! {
		pub enum Origin for Test	where system = frame_system {}
	}

	// For testing the pallet, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of pallets we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::one();
	}
	impl frame_system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type Call = ();
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type DbWeight = ();
		type BlockExecutionWeight = ();
		type ExtrinsicBaseWeight = ();
		type AvailableBlockRatio = AvailableBlockRatio;
		type MaximumBlockLength = MaximumBlockLength;
    type MaximumExtrinsicWeight = MaximumBlockWeight;
		type Version = ();
		type ModuleToIndex = ();
		type AccountData = ();
		type OnNewAccount = ();
		type OnKilledAccount = ();
	}
	impl Trait for Test {
		type Event = ();
		type Balance = u128;
	}
	type Assets = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sp_io::TestExternalities {
		frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn querying_total_supply_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::give_me(Origin::signed(1), 100));
			assert_eq!(Assets::balance(1), 100);
			assert_ok!(Assets::transfer(Origin::signed(1), 2, 50));
			assert_eq!(Assets::balance(1), 50);
			assert_eq!(Assets::balance(2), 50);
			assert_ok!(Assets::transfer(Origin::signed(2), 3, 31));
			assert_eq!(Assets::balance(1), 50);
			assert_eq!(Assets::balance(2), 19);
			assert_eq!(Assets::balance(3), 31);
			assert_eq!(Assets::total_supply(), 100);
		});
	}

	#[test]
	fn transferring_amount_above_available_balance_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::give_me(Origin::signed(1), 100));
			assert_eq!(Assets::balance(1), 100);
			assert_ok!(Assets::transfer(Origin::signed(1), 2, 50));
			assert_eq!(Assets::balance(1), 50);
			assert_eq!(Assets::balance(2), 50);
		});
	}

	#[test]
	fn transferring_amount_more_than_available_balance_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::give_me(Origin::signed(1), 100));
			assert_eq!(Assets::balance(1), 100);
			assert_ok!(Assets::transfer(Origin::signed(1), 2, 70));
			assert_eq!(Assets::balance(1), 30);
			assert_eq!(Assets::balance(2), 70);
			assert_noop!(Assets::transfer(Origin::signed(1), 1, 50), Error::<Test>::BalanceLow);
		});
	}

	#[test]
	fn transferring_less_than_one_unit_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::give_me(Origin::signed(1), 100));
			assert_eq!(Assets::balance(1), 100);
			assert_noop!(Assets::transfer(Origin::signed(1), 2, 0), Error::<Test>::AmountZero);
		});
	}

	#[test]
	fn transferring_more_units_than_total_supply_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::give_me(Origin::signed(1), 100));
			assert_eq!(Assets::balance(1), 100);
			assert_noop!(Assets::transfer(Origin::signed(1), 2, 101), Error::<Test>::BalanceLow);
		});
	}
}
