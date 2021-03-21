#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_error, 
	decl_event, 
	decl_module, 
	decl_storage, 
	ensure, 
	//dispatch::DispatchResult,
	traits::{
		Currency, 
		//Get,
		ReservableCurrency, 
		ExistenceRequirement::AllowDeath
	},
};
use frame_system::{
	self as system, 
	ensure_signed,
	ensure_root
};
use parity_scale_codec::{
	Decode, 
	Encode
};
use sp_std::prelude::*;

use pallet_token as Token;


#[cfg(test)]
mod tests;

pub trait Trait: system::Trait + pallet_token::Trait   {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Currency: ReservableCurrency<Self::AccountId>;
	// type AccountOperation: Get<Self::AccountId>;
	// type AccountVault: Get<Self::AccountId>;

}



pub type PairIndex = u128;
pub type TradeIndex = u128;
pub type BuyOrderIndex = u128;
pub type SellOrderIndex = u128;
pub type PairNativeIndex = u128;
pub type TradeNativeIndex = u128;
pub type BuyOrderNativeIndex = u128;
pub type SellOrderNativeIndex = u128;
pub type TokenIndex = u32;

type AccountIdOf<T> = <T as system::Trait>::AccountId;
type BalanceOf<T> = <<T as pallet_token::Trait>::Currency as Currency<AccountIdOf<T>>>::Balance;

type PairInfoOf<T> = PairInfo<AccountIdOf<T>, <T as system::Trait>::BlockNumber>;
type PairNativeInfoOf<T> = PairNativeInfo<AccountIdOf<T>, <T as system::Trait>::BlockNumber>;
type TradeInfoOf<T> = TradeInfo<AccountIdOf<T>, BalanceOf<T>, <T as system::Trait>::BlockNumber>;
type TradeNativeInfoOf<T> = TradeNativeInfo<AccountIdOf<T>, BalanceOf<T>, <T as system::Trait>::BlockNumber>;
type BuyOrderInfoOf<T> = BuyOrderInfo<AccountIdOf<T>, BalanceOf<T>, <T as system::Trait>::BlockNumber>;
type BuyOrderNativeInfoOf<T> = BuyOrderNativeInfo<AccountIdOf<T>, BalanceOf<T>, <T as system::Trait>::BlockNumber>;
type SellOrderInfoOf<T> = SellOrderInfo<AccountIdOf<T>, BalanceOf<T>, <T as system::Trait>::BlockNumber>;
type SellOrderNativeInfoOf<T> = SellOrderNativeInfo<AccountIdOf<T>, BalanceOf<T>, <T as system::Trait>::BlockNumber>;


#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PairInfo<AccountId, BlockNumber> {
	base: u32,
	target: u32,
	banker: AccountId,
	active: bool,
	created: BlockNumber
}

#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PairNativeInfo<AccountId, BlockNumber> {
	target: u32,
	banker: AccountId,
	active: bool,
	created: BlockNumber
}


#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct BuyOrderInfo<AccountId, Balance, BlockNumber> {
	order_id: BuyOrderIndex,
	pair: PairIndex,
	buyer: AccountId,
	volume: Balance,
	ratio: Balance,
	created: BlockNumber
}

#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct BuyOrderNativeInfo<AccountId, Balance, BlockNumber> {
	order_id: BuyOrderNativeIndex,
	pair: PairNativeIndex,
	buyer: AccountId,
	volume: Balance,
	ratio: Balance,
	created: BlockNumber
}

#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct SellOrderInfo<AccountId, Balance, BlockNumber> {
	order_id: SellOrderNativeIndex,
	pair: PairNativeIndex,
	seller: AccountId,
	volume: Balance,
	ratio: Balance,
	created: BlockNumber
}

#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct SellOrderNativeInfo<AccountId, Balance, BlockNumber> {
	order_id: SellOrderNativeIndex,
	pair: PairNativeIndex,
	seller: AccountId,
	volume: Balance,
	ratio: Balance,
	created: BlockNumber
}


#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TradeInfo<AccountId, Balance,  BlockNumber> {
	pair: PairIndex,
	seller: AccountId,
	buyer: AccountId,
	volume: Balance,
	ratio: Balance,
	created: BlockNumber
}

#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TradeNativeInfo<AccountId, Balance,  BlockNumber> {
	pair: PairNativeIndex,
	seller: AccountId,
	buyer: AccountId,
	volume: Balance,
	ratio: Balance,
	created: BlockNumber
}

decl_storage! {
	trait Store for Module<T: Trait> as Exchange {

		pub AccountOperation get(fn account_operation): AccountIdOf<T>;
		pub AccountVault get(fn account_vault): AccountIdOf<T>;
		
		pub MinimumRatio get(fn minimum_ratio): BalanceOf<T>;
		pub MinimumVolume get(fn minimum_volume): BalanceOf<T>;

		pub Pair get(fn pair): 
			map hasher(blake2_128_concat) PairIndex => Option<PairInfoOf<T>>;
		pub PairCount get(fn pair_count): PairIndex;	

		pub PairNative get(fn pair_native): 
			map hasher(blake2_128_concat) PairIndex => PairNativeInfoOf<T>;
		pub PairNativeCount get(fn pair_native_count): PairIndex;			

		pub BuyOrder get(fn buy_order): 
			map hasher(blake2_128_concat) (PairIndex, BuyOrderIndex) => BuyOrderInfoOf<T>;
		pub BuyOrderList get(fn buy_order_list): 
			map hasher(blake2_128_concat) PairIndex => Vec<BuyOrderIndex>;
		pub BuyOrderUserList get(fn buy_order_user_list): 
			map hasher(blake2_128_concat) (PairIndex, AccountIdOf<T>) => Vec<BuyOrderIndex>;			
		pub BuyOrderCount get(fn buy_order_count): 
			map hasher(blake2_128_concat) PairIndex => BuyOrderIndex;

		pub BuyOrderNative get(fn buy_order_native): 
			map hasher(blake2_128_concat) (PairNativeIndex, BuyOrderNativeIndex) => BuyOrderNativeInfoOf<T>;
		pub BuyOrderNativeList get(fn buy_order_native_list): 
			map hasher(blake2_128_concat) PairNativeIndex => Vec<BuyOrderNativeIndex>;
		pub BuyOrderNativeUserList get(fn buy_order_native_user_list): 
			map hasher(blake2_128_concat) (PairNativeIndex, AccountIdOf<T>) => Vec<BuyOrderNativeIndex>;				
		pub BuyOrderNativeCount get(fn buy_order_native_count): 
			map hasher(blake2_128_concat) PairNativeIndex => BuyOrderNativeIndex;

		pub SellOrder get(fn sell_order): 
			map hasher(blake2_128_concat) (PairIndex, SellOrderIndex) => SellOrderInfoOf<T>;
		pub SellOrderList get(fn sell_order_list): 
			map hasher(blake2_128_concat) PairIndex => Vec<SellOrderIndex>;
		pub SellOrderUserList get(fn sell_order_user_list): 
			map hasher(blake2_128_concat) (PairIndex, AccountIdOf<T>) => Vec<SellOrderIndex>;				
		pub SellOrderCount get(fn sell_order_count): 
			map hasher(blake2_128_concat) PairIndex => SellOrderIndex;

		pub SellOrderNative get(fn sell_order_native): 
			map hasher(blake2_128_concat) (PairNativeIndex, SellOrderNativeIndex) => SellOrderNativeInfoOf<T>;
		pub SellOrderNativeList get(fn sell_order_native_list): 
			map hasher(blake2_128_concat) PairNativeIndex => Vec<SellOrderNativeIndex>;
		pub SellOrderNativeUserList get(fn sell_order_native_user_list): 
			map hasher(blake2_128_concat) (PairNativeIndex, AccountIdOf<T>) => Vec<SellOrderNativeIndex>;				
		pub SellOrderNativeCount get(fn sell_order_native_count): 
			map hasher(blake2_128_concat) PairNativeIndex => SellOrderNativeIndex;

		pub Trades get(fn trades): 
			map hasher(blake2_128_concat) (PairIndex, TradeIndex) => Option<TradeInfoOf<T>>;
		pub TradeUserList get(fn trade_user_list): 
			map hasher(blake2_128_concat) (PairIndex, AccountIdOf<T>) => Vec<TradeIndex>;				
		pub TradeCount get(fn trade_count): 
			map hasher(blake2_128_concat) PairIndex => TradeIndex;
		
		pub TradeNatives get(fn trade_natives): 
			map hasher(blake2_128_concat) (PairNativeIndex, TradeNativeIndex) => Option<TradeNativeInfoOf<T>>;
		pub TradeNativeUserList get(fn trade_native_user_list): 
			map hasher(blake2_128_concat) (PairNativeIndex, AccountIdOf<T>) => Vec<TradeNativeIndex>;			
		pub TradeNativeCount get(fn trade_native_count): 
			map hasher(blake2_128_concat) PairNativeIndex => TradeNativeIndex;		
	}
}

decl_event! {
	pub enum Event<T> where
		Balance = BalanceOf<T>,
		//AccountId = <T as system::Trait>::AccountId,
		<T as system::Trait>::BlockNumber,
	{
		/// Pair successfully created. \[pair_id, block_number\]
		PairCreated(PairIndex, BlockNumber),
		/// Native Pair successfully created. \[pair_id, block_number\]
		PairNativeCreated(PairIndex, BlockNumber),	
		/// Native Pair successfully mutated. \[pair_id, block_number\]
		PairNativeMutated(PairIndex),				
		/// Pair is paused/unpause trading. \[pair_id, pause\]
		PairPaused(PairIndex, bool),
		/// Buy order successfully created. \[order_id, pair_id, ratio, volume\]
		BuyOrderCreated(BuyOrderIndex, PairIndex),
		/// Buy native order successfully created. \[order_id, pair_id, ratio, volume\]
		BuyOrderNativeCreated(BuyOrderIndex, PairIndex),		
		/// Sell order successfully created. \[order_id, pair_id, ratio, volume\]
		SellOrderCreated(SellOrderIndex, PairIndex),
		/// Sell native order successfully created. \[order_id, pair_id, ratio, volume\]
		SellOrderNativeCreated(SellOrderIndex, PairIndex),		
		/// Trade successfully created. \[trade_id, pair_id, ratio, volume\]
		TradeCreated(TradeIndex, PairIndex, Balance, Balance),
		/// Trade successfully created. \[trade_id, pair_id, ratio, volume\]
		TradeNativeCreated(TradeIndex, PairIndex, Balance, Balance),	
				
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Insuffiecient amount of token
		InsufficientAmountToSwap,
		/// Trading for pair is paused
		TradingPairPaused,

		NotTokenOwner,
		InsufficientAmount,
		InsufficientApproval,	

		BelowMinimumVolume,
		BelowMinimumRatio,
		
		NotExchangeAccount,
		NotOrderCreator,

	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		type Error = Error<T>;		
		
		#[weight = 10_000]
		fn set_accounts(
			origin, 
			account_type:u32, 
			account_id:AccountIdOf<T>
		) {
			ensure_root(origin)?;
			if account_type == 1 {
				<AccountOperation<T>>::put(account_id)
			} else if account_type == 2 {
				<AccountVault<T>>::put(account_id)
			}
		}

		#[weight = 10_000]
		fn set_fees(
			origin, 
			fee_type:u32, 
			fee:BalanceOf<T>
		) {
			let caller = ensure_signed(origin)?;
			let exchange_account = <AccountOperation<T>>::get();
			ensure!(caller == exchange_account, Error::<T>::NotExchangeAccount);
			
			if fee_type == 1 {
				<MinimumVolume<T>>::put(fee)
			} else if fee_type == 2 {
				<MinimumRatio<T>>::put(fee)
			}
		}		

		#[weight = 10_000]
		fn create_pair(
			origin,
			base: u32,
			target: u32
			) {		
			let banker = ensure_signed(origin.clone())?;
			let exchange_account = <AccountOperation<T>>::get();
			ensure!(banker == exchange_account, Error::<T>::NotExchangeAccount);

			let created = <system::Module<T>>::block_number();
			let active: bool = true;

			let index = PairCount::get();
			PairCount::put(index + 1);

			<Pair<T>>::insert(index, PairInfo {
				base,
				target,
				banker,
				active,
				created
			});

			Self::deposit_event(RawEvent::PairCreated(index, created));
		}	

		#[weight = 10_000]
		fn create_native_pair(
			origin,
			target: u32
			) {
			let banker = ensure_signed(origin)?;
			let exchange_account = <AccountOperation<T>>::get();
			ensure!(banker == exchange_account, Error::<T>::NotExchangeAccount);
			let created = <system::Module<T>>::block_number();
			let active: bool = true;

			let index = PairNativeCount::get();
			PairNativeCount::put(index + 1);

			<PairNative<T>>::insert(index, PairNativeInfo {
				target: target,
				banker: banker,
				active: active,
				created: created
			});

			Self::deposit_event(RawEvent::PairNativeCreated(index, created));
		}	
		
		#[weight = 10_000]
		fn edit_native_pair(
			origin,
			pair_index: u128,			
			target: u32
			) {
			let banker = ensure_signed(origin)?;
			let exchange_account = <AccountOperation<T>>::get();
			ensure!(banker == exchange_account, Error::<T>::NotExchangeAccount);

			let pair = <PairNative<T>>::get(pair_index);
			let pair_banker = pair.banker;
			let pair_active = pair.active;
			let pair_created = pair.created;

			<PairNative<T>>::mutate(pair_index, |v| *v = PairNativeInfo {
				target: target,
				banker: pair_banker,
				active: pair_active,
				created: pair_created
			});

			Self::deposit_event(RawEvent::PairNativeMutated(pair_index));
		}		
	
		#[weight = 10_000]
		fn buy(
			origin,
			pair: PairIndex,
			volume: BalanceOf<T>,
			ratio: BalanceOf<T>) {
			let creator = ensure_signed(origin)?;
			let caller = creator.clone();
			let caller2 = creator.clone();
			let created = <system::Module<T>>::block_number();	
			let base = <Pair<T>>::get(pair).unwrap().base;		
			let volume = volume;
			let ratio = ratio;

			let exchange = Self::account_operation();

			let base_balance = <Token::Module<T>>::get_balance(base, caller.clone());				
			ensure!(base_balance >= volume, Error::<T>::InsufficientAmount);

			let minimum_volume = Self::minimum_volume();
			ensure!(volume>= minimum_volume, Error::<T>::BelowMinimumVolume);

			let minimum_ratio = Self::minimum_ratio();
			ensure!(ratio>= minimum_ratio, Error::<T>::BelowMinimumRatio);			

			let _volume = volume;
			let _ratio = ratio;	

			<Token::Module<T>>::transfer_(base, caller.clone(), exchange.clone(), _volume.clone());					

			let index = <BuyOrderCount>::get(pair);			

			<BuyOrder<T>>::insert((pair, index), BuyOrderInfo {
				order_id: index,
				pair,
				buyer: caller,
				volume: _volume,
				ratio: _ratio,
				created
			});
			
			let mut buy_order_list = <BuyOrderList>::get(pair);
			buy_order_list.insert(buy_order_list.len(), index);
			<BuyOrderList>::insert(pair, buy_order_list);

			let mut buy_order_user_list = <BuyOrderUserList<T>>::get((pair, caller2.clone()));
			buy_order_user_list.insert(buy_order_user_list.len(), index);
			<BuyOrderUserList<T>>::insert((pair, caller2.clone()), buy_order_user_list);

			<BuyOrderCount>::insert(pair, index + 1);
			//Self::deposit_event(RawEvent::BuyOrderCreated(index, pair, _ratio, _volume));
		}	

		#[weight = 10_000]
		fn buy_native(
			origin,
			pair: PairIndex,
			volume: BalanceOf<T>,
			ratio: BalanceOf<T>) {
			let creator = ensure_signed(origin)?;
			let caller = creator.clone();
			let caller2 = creator.clone();
			let created = <system::Module<T>>::block_number();	
			
			let volume = volume;
			let ratio = ratio;

			let exchange = Self::account_operation();
			

			let _volume = volume;
			let _ratio = ratio;	

			Self::transfer_coin(caller.clone(), exchange.clone(), _volume.clone());		
			
			let minimum_volume = Self::minimum_volume();
			ensure!(volume>= minimum_volume, Error::<T>::BelowMinimumVolume);		
			
			let minimum_ratio = Self::minimum_ratio();
			ensure!(ratio>= minimum_ratio, Error::<T>::BelowMinimumRatio);				

			let index = <BuyOrderNativeCount>::get(pair);			

			<BuyOrderNative<T>>::insert((pair, index), BuyOrderNativeInfo {
				order_id: index,
				pair,
				buyer: caller,
				volume: _volume,
				ratio: _ratio,
				created
			});

			let mut buy_order_list = <BuyOrderNativeList>::get(pair);
			buy_order_list.insert(buy_order_list.len(), index);			
			<BuyOrderNativeList>::insert(pair, buy_order_list);

			let mut buy_order_user_list = <BuyOrderNativeUserList<T>>::get((pair, caller2.clone()));
			buy_order_user_list.insert(buy_order_user_list.len(), index);
			<BuyOrderNativeUserList<T>>::insert((pair, caller2.clone()), buy_order_user_list);

			<BuyOrderNativeCount>::insert(pair, index + 1);
		}			
	
		#[weight = 10_000]
		fn sell(
			origin,
			pair: PairIndex,
			volume: BalanceOf<T>,
			ratio: BalanceOf<T>) {

			let creator = ensure_signed(origin)?;
			let caller = creator.clone();
			let caller2 = creator.clone();
			let created = <system::Module<T>>::block_number();	
			let target = <Pair<T>>::get(pair).unwrap().target;	
			let volume = volume;
			let ratio = ratio;					
			let exchange = Self::account_operation();
			let target_balance = <Token::Module<T>>::get_balance(target, caller.clone());			

			ensure!(target_balance >= volume, Error::<T>::InsufficientAmount);

			let minimum_volume = Self::minimum_volume();
			ensure!(volume>= minimum_volume, Error::<T>::BelowMinimumVolume);	
			
			let minimum_ratio = Self::minimum_ratio();
			ensure!(ratio>= minimum_ratio, Error::<T>::BelowMinimumRatio);				

			let _volume = volume;
			let _ratio = ratio;	
			
			<Token::Module<T>>::transfer_(target, caller.clone(), exchange.clone(), _volume);		

			let index = <SellOrderCount>::get(pair);		
			<SellOrder<T>>::insert((pair, index), SellOrderInfo {
				order_id: index,
				pair,
				seller: caller,
				volume: _volume,
				ratio: _ratio,
				created
			});
			
			let mut sell_order_list = <SellOrderList>::get(pair);
			sell_order_list.insert(sell_order_list.len(), index);			
			<SellOrderList>::insert(pair, sell_order_list);

			let mut sell_order_user_list = <SellOrderUserList<T>>::get((pair, caller2.clone()));
			sell_order_user_list.insert(sell_order_user_list.len(), index);
			<SellOrderUserList<T>>::insert((pair, caller2.clone()), sell_order_user_list);			

			<SellOrderCount>::insert(pair, index + 1);	
			//Self::deposit_event(RawEvent::SellOrderCreated(index, pair, _ratio, _volume));
		}	
		
		#[weight = 10_000]
		fn sell_native(
			origin,
			pair: PairIndex,
			volume: BalanceOf<T>,
			ratio: BalanceOf<T>) {

			let creator = ensure_signed(origin)?;
			let caller = creator.clone();
			let caller2 = creator.clone();
			let created = <system::Module<T>>::block_number();	
			let target = <PairNative<T>>::get(pair).target;	
			let volume = volume;
			let ratio = ratio;					
			let exchange = Self::account_operation();
			let target_balance = <Token::Module<T>>::get_balance(target, caller.clone());			

			ensure!(target_balance >= volume, Error::<T>::InsufficientAmount);

			let minimum_volume = Self::minimum_volume();
			ensure!(volume>= minimum_volume, Error::<T>::BelowMinimumVolume);	

			let minimum_ratio = Self::minimum_ratio();
			ensure!(ratio>= minimum_ratio, Error::<T>::BelowMinimumRatio);				

			let _volume = volume;
			let _ratio = ratio;	
			
			<Token::Module<T>>::transfer_(target, caller.clone(), exchange.clone(), _volume);		

			let index = <SellOrderNativeCount>::get(pair);		
			<SellOrderNative<T>>::insert((pair, index), SellOrderNativeInfo {
				order_id: index,
				pair,
				seller: caller,
				volume: _volume,
				ratio: _ratio,
				created
			});
			
			let mut sell_order_list = <SellOrderNativeList>::get(pair);
			sell_order_list.insert(sell_order_list.len(), index);			
			<SellOrderNativeList>::insert(pair, sell_order_list);			

			let mut sell_order_user_list = <SellOrderNativeUserList<T>>::get((pair, caller2.clone()));
			sell_order_user_list.insert(sell_order_user_list.len(), index);
			<SellOrderNativeUserList<T>>::insert((pair, caller2.clone()), sell_order_user_list);			
			
			<SellOrderNativeCount>::insert(pair, index + 1);	
		}		
		
		#[weight = 10_000]
		fn cancel_buy(
			origin,
			pair: PairIndex,
			buy_order_id: BuyOrderIndex
		) {

			let creator = ensure_signed(origin)?;
			let caller = creator.clone();
			let buy_item = <BuyOrder<T>>::get((pair, buy_order_id));
			let buy_order_id = buy_item.order_id;
			let buy_volume = buy_item.volume;			
			let buyer = buy_item.buyer;	
			ensure!(creator == buyer, Error::<T>::NotOrderCreator);	
			let base = <Pair<T>>::get(pair).unwrap().base;
			let exchange = Self::account_operation();
			let vault = Self::account_vault();
			let p999 = sp_runtime::Permill::from_parts(999000);
	
			let base_after_fee = p999 * buy_volume;		
			let volume_buyer = base_after_fee;
			let volume_vault = buy_volume - volume_buyer;
		
			let mut buy_order_list = <BuyOrderList>::get(pair);

			match buy_order_list.binary_search(&buy_order_id) {

				Ok(index) => {
					buy_order_list.remove(index);
					<BuyOrderList>::insert(pair, buy_order_list);

					<BuyOrder<T>>::remove((pair, buy_order_id));	
		
					<Token::Module<T>>::transfer_(base, exchange.clone(), caller.clone(), volume_buyer);							
					<Token::Module<T>>::transfer_(base, exchange.clone(), vault.clone(), volume_vault);							
				},
				Err(_) => {
					
				},
			}

			let mut buy_order_user_list = <BuyOrderUserList<T>>::get((pair, buyer.clone()));

			match buy_order_user_list.binary_search(&buy_order_id) {

				Ok(index) => {
					buy_order_user_list.remove(index);
					<BuyOrderUserList<T>>::insert((pair, buyer.clone()), buy_order_user_list);
				},
				Err(_) => {
					
				},
			}			


		}	
		
		#[weight = 10_000]
		fn cancel_native_buy(
			origin,
			pair: PairNativeIndex,
			buy_order_id: BuyOrderNativeIndex
		) {
			let creator = ensure_signed(origin)?;
			let buy_item = <BuyOrderNative<T>>::get((pair, buy_order_id));
			let buy_order_id = buy_item.order_id;
			let buy_volume = buy_item.volume;			
			let buyer = buy_item.buyer;		
			ensure!(creator == buyer, Error::<T>::NotOrderCreator);
			let exchange = Self::account_operation();
			let vault = Self::account_vault();
			let p999 = sp_runtime::Permill::from_parts(999000);
	
			let base_after_fee = p999 * buy_volume;		
			let volume_buyer = base_after_fee;
			let volume_vault = buy_volume - volume_buyer;
		
			let mut buy_order_list = <BuyOrderNativeList>::get(pair);

			match buy_order_list.binary_search(&buy_order_id) {

				Ok(index) => {
					buy_order_list.remove(index);
					<BuyOrderNativeList>::insert(pair, buy_order_list);

					<BuyOrderNative<T>>::remove((pair, buy_order_id));	

					Self::transfer_coin(exchange.clone(), buyer.clone(), volume_buyer);
					Self::transfer_coin(exchange.clone(), vault.clone(), volume_vault);											
				},
				Err(_) => {
					
				},
			}

			let mut buy_order_user_list = <BuyOrderNativeUserList<T>>::get((pair, buyer.clone()));

			match buy_order_user_list.binary_search(&buy_order_id) {

				Ok(index) => {
					buy_order_user_list.remove(index);
					<BuyOrderNativeUserList<T>>::insert((pair, buyer), buy_order_user_list);
				},
				Err(_) => {
					
				},
			}			

		}	
		
		#[weight = 10_000]
		fn cancel_sell(
			origin,
			pair: PairIndex,
			sell_order_id: SellOrderIndex
		) {
			let creator = ensure_signed(origin)?;
			let caller = creator.clone();
			let sell_item = <SellOrder<T>>::get((pair, sell_order_id));
			let sell_order_id = sell_item.order_id;
			let sell_volume = sell_item.volume;			
			let seller = sell_item.seller;		
			ensure!(creator == seller, Error::<T>::NotOrderCreator );	
			let target = <Pair<T>>::get(pair).unwrap().target;		
			let exchange = Self::account_operation();
			let vault = Self::account_vault();
			let p999 = sp_runtime::Permill::from_parts(999000);
	
			let base_after_fee = p999 * sell_volume;		
			let volume_seller = base_after_fee;
			let volume_vault = sell_volume - volume_seller;
		
			let mut sell_order_list = <SellOrderList>::get(pair);

			match sell_order_list.binary_search(&sell_order_id) {

				Ok(index) => {
					sell_order_list.remove(index);
					<SellOrderList>::insert(pair, sell_order_list);

					<SellOrder<T>>::remove((pair, sell_order_id));	
		
					<Token::Module<T>>::transfer_(target, exchange.clone(), caller.clone(), volume_seller);							
					<Token::Module<T>>::transfer_(target, exchange.clone(), vault.clone(), volume_vault);							
				},
				Err(_) => {
					
				},
			}

			let mut sell_order_user_list = <SellOrderUserList<T>>::get((pair, seller.clone()));

			match sell_order_user_list.binary_search(&sell_order_id) {

				Ok(index) => {
					sell_order_user_list.remove(index);
					<SellOrderUserList<T>>::insert((pair, seller.clone()), sell_order_user_list);
				},
				Err(_) => {
					
				},
			}			


		}	
		
		#[weight = 10_000]
		fn cancel_native_sell(
			origin,
			pair: PairNativeIndex,
			sell_order_id: SellOrderNativeIndex
		) {
			let creator = ensure_signed(origin)?;
			let caller = creator.clone();
			let sell_item = <SellOrderNative<T>>::get((pair, sell_order_id));
			let sell_order_id = sell_item.order_id;
			let sell_volume = sell_item.volume;			
			let seller = sell_item.seller;	
			ensure!(creator == seller, Error::<T>::NotOrderCreator );	
			let target = <PairNative<T>>::get(pair).target;	
			let exchange = Self::account_operation();
			let vault = Self::account_vault();
			let p999 = sp_runtime::Permill::from_parts(999000);
	
			let base_after_fee = p999 * sell_volume;		
			let volume_seller = base_after_fee;
			let volume_vault = sell_volume - volume_seller;
		
			let mut sell_order_list = <SellOrderNativeList>::get(pair);

			match sell_order_list.binary_search(&sell_order_id) {

				Ok(index) => {
					sell_order_list.remove(index);
					<SellOrderNativeList>::insert(pair, sell_order_list);

					<SellOrderNative<T>>::remove((pair, sell_order_id));	

					<Token::Module<T>>::transfer_(target, exchange.clone(), caller.clone(), volume_seller);							
					<Token::Module<T>>::transfer_(target, exchange.clone(), vault.clone(), volume_vault);											
				},
				Err(_) => {
					
				},
			}

			let mut sell_order_user_list = <SellOrderNativeUserList<T>>::get((pair, seller.clone()));

			match sell_order_user_list.binary_search(&sell_order_id) {

				Ok(index) => {
					sell_order_user_list.remove(index);
					<SellOrderNativeUserList<T>>::insert((pair, seller.clone()), sell_order_user_list);
				},
				Err(_) => {
					
				},
			}			

		}			
		
							
		fn on_finalize(now: T::BlockNumber) {
			
			let _now = now;
			Self::match_native_orders();
			Self::match_orders();

		}

	}
}

impl<T: Trait> Module<T> {

	fn transfer_coin( 
		from: AccountIdOf<T>, 
		to: AccountIdOf<T>, 
		value:BalanceOf<T>) -> () {

		let _lol = <T as pallet_token::Trait>::Currency::transfer(&from, &to, value, AllowDeath);//.map_err(|_| DispatchError::Other("Can't make transfer"))?;			
	}	


	fn swap(pair: u128, 
		seller: AccountIdOf<T>, 
		buyer: AccountIdOf<T>,
		seller_volume: BalanceOf<T>, 
		buyer_volume: BalanceOf<T>, 
		ratio:BalanceOf<T>) -> () {
		
		let exchange = Self::account_operation();
		let vault = Self::account_vault();
		let base = <Pair<T>>::get(pair).unwrap().base;
		let target = <Pair<T>>::get(pair).unwrap().target;
		let p999 = sp_runtime::Permill::from_parts(999000);

		let base_after_fee = p999 * seller_volume;		
		<Token::Module<T>>::transfer_(base, exchange.clone(), seller.clone(), base_after_fee);
		<Token::Module<T>>::transfer_(base, exchange.clone(), vault.clone(), seller_volume - base_after_fee);
		let target_after_fee = p999 * buyer_volume;
		<Token::Module<T>>::transfer_(target, exchange.clone(), buyer.clone(), target_after_fee);
		<Token::Module<T>>::transfer_(target, exchange.clone(), vault.clone(), buyer_volume - target_after_fee);

		let created = <system::Module<T>>::block_number();	
		let index = <TradeCount>::get(pair);	

		<Trades<T>>::insert((pair, index), TradeInfo {
			pair,
			seller,
			buyer,
			volume: buyer_volume,
			ratio,
			created,
		});		
		<TradeCount>::mutate(pair, |v| *v += 1);		
		Self::deposit_event(RawEvent::TradeCreated(index, pair, ratio, buyer_volume));			
	}

	fn match_orders() -> () {

		let all_pairs = <PairCount>::get();
		let min_volume = <MinimumVolume<T>>::get();
		let pre_: BalanceOf<T> = 1000000.into();
		let rationalisation: BalanceOf<T> = pre_* pre_;	
		
		for pair in 0..all_pairs {

			let _buy_orders = <BuyOrderList>::get(pair);
			let _sell_orders = <SellOrderList>::get(pair);

			let buy_orders_iter = _buy_orders.iter();				

			for buy_item_number in buy_orders_iter {
				let buy_item = <BuyOrder<T>>::get((pair, buy_item_number));

				let sell_orders_iter = _sell_orders.iter();

				for sell_item_number in sell_orders_iter {

					let sell_item = <SellOrder<T>>::get((pair, sell_item_number));

					let buy_account = &buy_item.buyer;
					let buy_order_id = buy_item.order_id;
					let buy_volume = buy_item.volume;
					let buy_price = buy_item.ratio;	
					let buy_created = &buy_item.created;		

					let sell_account = &sell_item.seller;
					let sell_order_id = sell_item.order_id;
					let sell_volume = sell_item.volume;
					let sell_price = sell_item.ratio;
					let sell_created = &sell_item.created;	

					let trade_price = buy_price;

					if buy_price >= sell_price && buy_volume >= min_volume && sell_volume >= min_volume {
						
						let buy_implied_volume = buy_volume * buy_price / rationalisation;
						let sell_implied_volume = sell_price / sell_volume * rationalisation;
						let base_volume;
						let target_volume;						

						if sell_volume < buy_implied_volume {
							base_volume = sell_implied_volume;
							target_volume = sell_volume;
						} else {
							base_volume = buy_implied_volume;
							target_volume = buy_volume;
						}

						let new_buyer_volume = buy_volume - base_volume;
						let new_seller_volume =  sell_volume - target_volume;

						BuyOrder::<T>::mutate((pair, buy_order_id), |v| *v = BuyOrderInfo {
							order_id: buy_order_id,
							pair: pair,
							buyer: buy_account.clone(),
							volume: new_buyer_volume,
							ratio: buy_price,
							created: *buy_created
						});
						
						SellOrder::<T>::mutate((pair, sell_order_id), |v| *v = SellOrderInfo {
							order_id: sell_order_id,
							pair: pair,
							seller: sell_account.clone(),
							volume: new_seller_volume,
							ratio: sell_price,
							created: *sell_created
						});									

						// SWAP
						Self::swap(pair.clone(), 
							sell_account.clone(), 
							buy_account.clone(), 
							base_volume.clone(), 
							target_volume.clone(), 
							trade_price);	
						// Reduce volume
					} else {
						
					}
					

					// if buy_price >= sell_price {

					// 	let matched_price = buy_price;
					// 	let pair: u128 = pair.into();
					// 	let mut seller_volume = sell_volume;
					// 	let mut buyer_volume =  buy_volume;
					// 	let mut matched_volume = sell_volume;

					// 	if buy_volume >= sell_volume {
					// 		matched_volume = sell_volume
					// 		seller_volume -= sell_volume;
					// 		buyer_volume = buy_volume - sell_volume;

					// 	} else if buy_volume < sell_volume && buy_volume > min_volume {
					// 		matched_volume = buy_volume;
					// 		seller_volume -= buy_volume;
					// 		buyer_volume = sell_volume - buy_volume;	

					// 	} 
					
																					

					// } 						
				}	

			}
		

			let buy_orders_iter2 = _buy_orders.iter();	
			for buy_item_number in buy_orders_iter2 {
				let buy_item = <BuyOrder<T>>::get((pair, buy_item_number));
				let buy_order_id = buy_item.order_id;
				let buy_volume = buy_item.volume;		
				let buyer = buy_item.buyer;			

				if buy_volume <= min_volume {					
					let mut buy_order_list = <BuyOrderList>::get(pair);

					match buy_order_list.binary_search(&buy_item_number) {

						Ok(index) => {
							buy_order_list.remove(index);
							<BuyOrderList>::insert(pair, buy_order_list);

							<BuyOrder<T>>::remove((pair, buy_order_id));
						},
						Err(_) => {
							
						},
					}

					let mut buy_order_user_list = <BuyOrderUserList<T>>::get((pair, buyer.clone()));

					match buy_order_user_list.binary_search(&buy_item_number) {

						Ok(index) => {
							buy_order_user_list.remove(index);
							<BuyOrderUserList<T>>::insert((pair, buyer.clone()), buy_order_user_list);
						},
						Err(_) => {
							
						},
					}					
				}		
			}				

			let sell_orders_iter2 = _sell_orders.iter();

			for sell_item_number in sell_orders_iter2 {
				let sell_item = <SellOrder<T>>::get((pair, sell_item_number));	
				let sell_order_id = sell_item.order_id;
				let sell_volume = sell_item.volume;	
				let seller = sell_item.seller;
				
				if sell_volume <= min_volume {					
					let mut sell_order_list = <SellOrderList>::get(pair);
					match sell_order_list.binary_search(&sell_item_number) {

						Ok(index) => {
							sell_order_list.remove(index);
							<SellOrderList>::insert(pair, sell_order_list);							
							<SellOrder<T>>::remove((pair, sell_order_id));
						},
						Err(_) => {
							
						},
					}

					let mut sell_order_user_list = <SellOrderUserList<T>>::get((pair, seller.clone()));

					match sell_order_user_list.binary_search(&sell_item_number) {

						Ok(index) => {
							sell_order_user_list.remove(index);
							<SellOrderUserList<T>>::insert((pair,seller.clone()), sell_order_user_list);
						},
						Err(_) => {
							
						},
					}						
				}					

				//SellOrderCount::mutate(pair, |v| *v -= 1);	
			}				
					
	
		}		
	}

	fn swap_native(pair: u128, 
		seller: AccountIdOf<T>, 
		buyer: AccountIdOf<T>,
		seller_volume: BalanceOf<T>, 
		buyer_volume: BalanceOf<T>, 
		ratio:BalanceOf<T>) -> () {
		
		let exchange = Self::account_operation();
		let vault = Self::account_vault();
		let target = <PairNative<T>>::get(pair).target;
		let p999 = sp_runtime::Permill::from_parts(999000);

		let base_after_fee = p999 * seller_volume;		
		Self::transfer_coin(exchange.clone(), seller.clone(), base_after_fee);
		Self::transfer_coin(exchange.clone(), vault.clone(), seller_volume - base_after_fee);
		let target_after_fee = p999 * buyer_volume;
		<Token::Module<T>>::transfer_(target, exchange.clone(), buyer.clone(), target_after_fee);
		<Token::Module<T>>::transfer_(target, exchange.clone(), vault.clone(), buyer_volume - target_after_fee);

		let created = <system::Module<T>>::block_number();	
		let index = <TradeNativeCount>::get(pair);	

		<TradeNatives<T>>::insert((pair, index), TradeNativeInfo {
			pair,
			seller,
			buyer,
			volume: buyer_volume,
			ratio,
			created,
		});	
		<TradeNativeCount>::mutate(pair, |v| *v += 1);		
		Self::deposit_event(RawEvent::TradeNativeCreated(index, pair, ratio, buyer_volume));			
	}

	fn match_native_orders() -> () {

		let all_pairs = <PairNativeCount>::get();
		let min_volume = <MinimumVolume<T>>::get();
		let pre_: BalanceOf<T> = 1000000.into();
		let rationalisation: BalanceOf<T> = pre_* pre_;	
		
		for pair in 0..all_pairs {

			let _buy_orders = <BuyOrderNativeList>::get(pair);
			let _sell_orders = <SellOrderNativeList>::get(pair);

			let buy_orders_iter = _buy_orders.iter();				

			for buy_item_number in buy_orders_iter {
				let buy_item = <BuyOrderNative<T>>::get((pair, buy_item_number));

				let sell_orders_iter = _sell_orders.iter();

				for sell_item_number in sell_orders_iter {
					let sell_item = <SellOrderNative<T>>::get((pair, sell_item_number));

					let buy_account = &buy_item.buyer;
					let buy_order_id = buy_item.order_id;
					let buy_volume = buy_item.volume;
					let buy_price = buy_item.ratio;	
					let buy_created = &buy_item.created;		

					let sell_account = &sell_item.seller;
					let sell_order_id = sell_item.order_id;
					let sell_volume = sell_item.volume;
					let sell_price = sell_item.ratio;
					let sell_created = &sell_item.created;	

					let trade_price = buy_price;

					if buy_price >= sell_price && buy_volume >= min_volume && sell_volume >= min_volume {
						
						let buy_implied_volume = buy_volume * buy_price / rationalisation;
						let sell_implied_volume = sell_price / sell_volume * rationalisation;
						let base_volume;
						let target_volume;						

						if sell_volume < buy_implied_volume {
							base_volume = sell_implied_volume;
							target_volume = sell_volume;
						} else {
							base_volume = buy_implied_volume;
							target_volume = buy_volume;
						}

						let new_buyer_volume = buy_volume - base_volume;
						let new_seller_volume =  sell_volume - target_volume;

						BuyOrderNative::<T>::mutate((pair, buy_order_id), |v| *v = BuyOrderNativeInfo {
							order_id: buy_order_id,
							pair: pair,
							buyer: buy_account.clone(),
							volume: new_buyer_volume,
							ratio: buy_price,
							created: *buy_created
						});
						
						SellOrderNative::<T>::mutate((pair, sell_order_id), |v| *v = SellOrderNativeInfo {
							order_id: sell_order_id,
							pair: pair,
							seller: sell_account.clone(),
							volume: new_seller_volume,
							ratio: sell_price,
							created: *sell_created
						});									

						// SWAP
						Self::swap_native(pair.clone(), 
							sell_account.clone(), 
							buy_account.clone(), 
							base_volume.clone(), 
							target_volume.clone(), 
							trade_price);	
						// Reduce volume
					} else {
						
					}
					

					
				}	

			}


			let buy_orders_iter2 = _buy_orders.iter();	
			for buy_item_number in buy_orders_iter2 {
				let buy_item = <BuyOrderNative<T>>::get((pair, buy_item_number));
				let buy_order_id = buy_item.order_id;
				let buy_volume = buy_item.volume;	
				let buyer = buy_item.buyer;				

				if buy_volume <= min_volume {					
					let mut buy_order_list = <BuyOrderNativeList>::get(pair);
					match buy_order_list.binary_search(&buy_item_number) {

						Ok(index) => {
							buy_order_list.remove(index);
							<BuyOrderNativeList>::insert(pair, buy_order_list);							
							<BuyOrderNative<T>>::remove((pair, buy_order_id));
						},
						Err(_) => {
							
						},
					}

					let mut buy_order_user_list = <BuyOrderNativeUserList<T>>::get((pair, buyer.clone()));

					match buy_order_user_list.binary_search(&buy_item_number) {

						Ok(index) => {
							buy_order_user_list.remove(index);
							<BuyOrderNativeUserList<T>>::insert((pair, buyer.clone()), buy_order_user_list);
						},
						Err(_) => {
							
						},
					}						
				}	
			}				

			let sell_orders_iter2 = _sell_orders.iter();

			for sell_item_number in sell_orders_iter2 {
				let sell_item = <SellOrderNative<T>>::get((pair, sell_item_number));
				let sell_order_id = sell_item.order_id;
				let sell_volume = sell_item.volume;		
				let seller = sell_item.seller;			

				if sell_volume <= min_volume {					
					let mut sell_order_list = <SellOrderNativeList>::get(pair);
					match sell_order_list.binary_search(&sell_item_number) {

						Ok(index) => {
							sell_order_list.remove(index);
							<SellOrderNativeList>::insert(pair, sell_order_list);							
							<SellOrderNative<T>>::remove((pair, sell_order_id));
						},
						Err(_) => {
							
						},
					}

					let mut sell_order_user_list = <SellOrderNativeUserList<T>>::get((pair, seller.clone()));

					match sell_order_user_list.binary_search(&sell_item_number) {

						Ok(index) => {
							sell_order_user_list.remove(index);
							<SellOrderNativeUserList<T>>::insert((pair,seller.clone()), sell_order_user_list);
						},
						Err(_) => {
							
						},
					}						
				}	
			}				
					
	
		}		
	}
	
}

