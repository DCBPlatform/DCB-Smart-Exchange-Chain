#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, ensure, dispatch::DispatchResult,
	traits::{
		Currency, 
		ReservableCurrency, 
	},
};
use frame_system::{self as system, ensure_signed, ensure_root};
use parity_scale_codec::{Decode, Encode};
use sp_std::prelude::*;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Currency: ReservableCurrency<Self::AccountId>;
}

pub type TokenIndex = u32;

type AccountIdOf<T> = <T as system::Trait>::AccountId;
type BalanceOf<T> = <<T as Trait>::Currency as Currency<AccountIdOf<T>>>::Balance;
type TokenInfoOf<T> = TokenInfo<AccountIdOf<T>, <T as system::Trait>::BlockNumber>;
//type LockedTokenInfoOf<T> = LockedTokenInfo<AccountIdOf<T>, <T as system::Trait>::BlockNumber>;
//type ReservedTokenInfoOf<T> = ReservedTokenInfo<AccountIdOf<T>, <T as system::Trait>::BlockNumber>;

#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TokenInfo<AccountId, BlockNumber> {
	name: Vec<u8>,
	symbol: Vec<u8>,	
	owner: AccountId,
	created: BlockNumber,
}

// #[derive(Encode, Decode, Default, PartialEq, Eq)]
// #[cfg_attr(feature = "std", derive(Debug))]
// pub struct LockedTokenInfo<AccountId, BlockNumber> {
// 	token: TokenIndex,
// 	from: BlockNumber,	
// 	to: BlockNumber,	
// 	account: AccountId,
// 	created: BlockNumber,
// }

// #[derive(Encode, Decode, Default, PartialEq, Eq)]
// #[cfg_attr(feature = "std", derive(Debug))]
// pub struct ReservedTokenInfo<AccountId, BlockNumber> {
// 	token: TokenIndex,
// 	reserved_by: AccountId,
// 	account: AccountId,
// 	created: BlockNumber,
// }

decl_storage! {
	trait Store for Module<T: Trait> as TokenStore {

		pub Tokens get(fn tokens): map hasher(blake2_128_concat) TokenIndex => TokenInfoOf<T>;
		pub TokenCount get(fn token_count): TokenIndex;

		pub Balance get(fn balance): map hasher(blake2_128_concat) (u32, T::AccountId) => BalanceOf<T>;
		pub Freezed get(fn freezed): map hasher(blake2_128_concat) (u32, T::AccountId) => bool;
		pub Supply get(fn supply): map hasher(blake2_128_concat) u32 => BalanceOf<T>;
		pub Paused get(fn paused): map hasher(blake2_128_concat) u32 => bool;
		pub Allowance get(fn allowance): map hasher(blake2_128_concat) (u32, T::AccountId, T::AccountId) => BalanceOf<T>;
		pub Owner get(fn owner): map hasher(blake2_128_concat) u32 => T::AccountId;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
		Balance = BalanceOf<T>,
	{
		/// A token was created by user. \[token_id, owner_id\]
		Created(u32, AccountId),
		/// Token burned. \[token, sender, amount\]
		Burn(u32, AccountId, Balance),
		/// Token minted. \[token, receiver, amount\]
		Mint(u32, AccountId, Balance),
		/// Token edited. \[token\]
		Edited(u32),		
		/// Token freezed. \[token, user\]
		Freeze(u32, AccountId),
		/// Token thawed. \[token, user\]
		Thaw(u32, AccountId),				
		/// Token transferred. \[token, sender, receiver, amount\]
		Transfer(u32, AccountId, AccountId, Balance),	
		/// Token transferred. \[token, user, spender, amount\]
		Spend(u32, AccountId, AccountId, Balance),				
		/// Token approved. \[token, user, spender amount\]
		Allowance(u32, AccountId, AccountId, Balance),
		/// Token paused/unpaused. \[token, status\]
		TokenPaused(u32, bool),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		NotTokenOwner,
		InsufficientAmount,
		InsufficientAllowance,
		InsufficientBalance,
		TokenPaused,
		AccountFreezed
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		type Error = Error<T>;	

		#[weight = 10_000]
		pub fn create(origin, 
			owner:AccountIdOf<T>, 
			name:Vec<u8>, 
			symbol: Vec<u8>, 
			initial_supply: BalanceOf<T>
		) -> DispatchResult {			
			let caller = ensure_signed(origin)?;

			let index = TokenCount::get();
			TokenCount::put(index + 1);		
			
			let created = <system::Module<T>>::block_number();
			Self::deposit_event(RawEvent::Created(index, owner.clone()));

			<Tokens<T>>::insert(index, TokenInfo {
				name,
				symbol,
				owner,
				created
			});			

			<Balance<T>>::insert((index, &caller), initial_supply);
			<Supply<T>>::insert(index, initial_supply);
			<Owner<T>>::insert(index, &caller);


			Ok(())
		}	
		
		#[weight = 10_000]
		pub fn transfer(origin, 
			token:u32, 
			to: T::AccountId, 
			value: BalanceOf<T> 
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			let from_balance = Self::balance((token, &from));
			ensure!(from_balance > value, <Error<T>>::InsufficientBalance);
			
			let paused = Self::paused(token);	
			ensure!(paused != true, <Error<T>>::TokenPaused);
			
			let freezed = Self::freezed((token, &from));
			ensure!(freezed != true, <Error<T>>::AccountFreezed);

			Self::transfer_(token, from, to, value);
			Ok(())
		}	

		#[weight = 10_000]
		pub fn spend(origin, 
			token:u32, 
			user: T::AccountId, 
			value: BalanceOf<T> 
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			let user_balance = Self::balance((token, &user));
			ensure!(user_balance >= value, <Error<T>>::InsufficientBalance);
			
			let caller_allowance = Self::allowance((token, &user, &caller));
			ensure!(caller_allowance >= value, <Error<T>>::InsufficientAllowance);
			
			let paused = Self::paused(token);	
			ensure!(paused != true, <Error<T>>::TokenPaused);
			
			let freezed = Self::freezed((token, &user));
			ensure!(freezed != true, <Error<T>>::AccountFreezed);

			Self::spend_(token, user, caller, value);
			Ok(())
		}	
		
		#[weight = 10_000]
		pub fn edit(origin, 
			token: u32, 
			name:Vec<u8>, 
			symbol: Vec<u8>
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			let token_owner = Self::owner(token);
			ensure!(caller == token_owner, <Error<T>>::NotTokenOwner);

			let token_data = <Tokens<T>>::get(token);
			let token_owner = token_data.owner;
			let token_created = token_data.created;

			<Tokens<T>>::mutate(token, |v| *v = TokenInfo {
				name,
				symbol,
				owner: token_owner,
				created: token_created
			});					

		
			Self::deposit_event(RawEvent::Edited(token.clone()));
			Ok(())
		}		
				
		#[weight = 10_000]
		pub fn pause(origin, 
			token: u32, 
			status: bool 
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			let token_owner = Self::owner(token);
			ensure!(caller == token_owner, <Error<T>>::NotTokenOwner);

			let token_boolean = Self::paused(token);
			let new_status: bool;
			if token_boolean {
				new_status = true;
			} else {	
				new_status = false;			
			}
			<Paused>::insert(token, new_status);			
			Self::deposit_event(RawEvent::TokenPaused(token, new_status));
			Ok(())
		}	
		
		#[weight = 10_000]
		pub fn mint(origin, 
			token:u32, 
			value: BalanceOf<T> 
		) -> DispatchResult {
			let minter = ensure_signed(origin)?;
			let token_owner = Self::owner(token);
			ensure!(minter == token_owner, <Error<T>>::NotTokenOwner);	
			let minter_balance = Self::balance((token, &minter));
			let token_supply = Self::supply(token);
			<Balance<T>>::insert((token, &minter), minter_balance + value);
			<Supply<T>>::insert(token, token_supply + value);
	
			Self::deposit_event(RawEvent::Mint(token, minter, value));					
			Ok(())
		}	
		
		#[weight = 10_000]
		pub fn burn(origin, 
			token:u32, 
			value: BalanceOf<T> 
		) -> DispatchResult {
			let burner = ensure_signed(origin)?;
			let token_owner = Self::owner(token);
			ensure!(burner == token_owner, <Error<T>>::NotTokenOwner);			
			let burner_balance = Self::balance((token, &burner));
			let token_supply = Self::supply(token);
	
			<Balance<T>>::insert((token, &burner), burner_balance - value);
			<Supply<T>>::insert(token, token_supply - value);
	
			Self::deposit_event(RawEvent::Burn(token, burner, value));
			Ok(())
		}	

		#[weight = 10_000]
		pub fn freeze(origin, 
			user: T::AccountId, 
			token:u32, 
			value: BalanceOf<T> 
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			let token_owner = Self::owner(token);
			ensure!(caller == token_owner, <Error<T>>::NotTokenOwner);	

			<Freezed<T>>::insert((token, &user), true);
	
			Self::deposit_event(RawEvent::Freeze(token, user));					
			Ok(())
		}	

		#[weight = 10_000]
		pub fn thaw(origin, 
			user: T::AccountId, 
			token:u32, 
			value: BalanceOf<T> 
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			let token_owner = Self::owner(token);
			ensure!(caller == token_owner, <Error<T>>::NotTokenOwner);	

			<Freezed<T>>::insert((token, &user), false);
	
			Self::deposit_event(RawEvent::Thaw(token, user));					
			Ok(())
		}		

		#[weight = 10_000]
		pub fn allow(origin, 
			token:u32, 
			spender:T::AccountId,
			value: BalanceOf<T> 
		) -> DispatchResult {
			let user = ensure_signed(origin)?;		
			<Allowance<T>>::insert((token, &user, &spender), value);
			Self::deposit_event(RawEvent::Allowance(token, user, spender, value));
			Ok(())
		}			

	
	}
}

impl<T: Trait> Module<T> {

	pub fn spend_(token: u32, user: AccountIdOf<T>, spender: AccountIdOf<T>, value: BalanceOf<T> ) -> () {
		let user_balance = Self::balance((token, &user));
		let spender_balance = Self::balance((token, &spender));

		<Balance<T>>::insert((token, &user), user_balance - value);
		<Balance<T>>::insert((token, &spender), spender_balance + value);
		Self::deposit_event(RawEvent::Spend(token, user, spender, value));
	
	}	

	pub fn transfer_(token: u32, from: AccountIdOf<T>, to: AccountIdOf<T>, value: BalanceOf<T> ) -> () {
		let from_balance = Self::balance((token, &from));
		let to_balance = Self::balance((token, &to));		

		<Balance<T>>::insert((token, &from), from_balance - value);
		<Balance<T>>::insert((token, &to), to_balance + value);
		Self::deposit_event(RawEvent::Transfer(token, from, to, value));

	}

	pub fn get_allowance(token: u32, user: AccountIdOf<T>, spender: AccountIdOf<T> ) -> BalanceOf<T> {
		Self::allowance((token, user, spender))
	}		

	pub fn get_balance(token: u32, who: AccountIdOf<T> ) -> BalanceOf<T> {
		Self::balance((token, who))
	}
	
	pub fn get_account_freezed(token: u32, who: AccountIdOf<T> ) -> bool {
		Self::freezed((token, who))
	}	


}
