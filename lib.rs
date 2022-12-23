#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;

    pub trait Config: frame_system::Config{
        #[pallet::constant]
        type MaxClaimLength: Get<u32>;
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>
    }

    #[pallet::pallet]
    #[pallet::generate_store{pub(super) trait Store}]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type Proofs<T: config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxClaimLength>,
        {T::AccountId, T::BlockNumber},
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config>{
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T>{
        ProofAlreadyExist,
        ClaimTooLong,
        ClaimNotExist,
        NotClaimOwner,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T>{
        #[pallet::weight(0)]
        pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo{
            		// 做必要检查，检查内容： 1，交易发送方是不是一个签名的用户 2，存证是否被别人创建过，创建过就抛出错误
			// 首先去创建签名交易，通过ensure_signed这样的system提供的版本方法来校验
			let sender = ensure_signed(origin)?;  // 存证拥有人是交易发送方，只有拥有人才可以调用存证，sender即当前交易发送方
            // 如果存在存证，返回错误 ProofAlreadyExist
            // ps:ensure!宏是确保表达式中的结果为true，这里取反操作
          ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist);  // 这里用到一个错误  ProofAlreadyExist，该错误需要在decl_error声明
          // 做insert操作，insert是key-value方式。这里的key-value是一个tuple
          // 这个tuple的第一个元素是AccountId；第二个是当前交易所处的区块，使用系统模块提供的block_number工具方法获取
          Proofs::<T>::insert(&claim,(sender.clone(),system::Module::<T>::block_number()));  // 插入操作
          // 触发一个event来通知客户端，RawEvent由宏生成；   sender:存在拥有人；claim:存在hash值 通过event通知客户端
          Self::deposit_event(RawEvent::ClaimCreated(sender,claim));   // ClaimCreated事件，需要decl_event处理
          // 返回ok
          Ok(())
        }

        #[pallet::weight(0)]
        pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;  // 交易发送方式已签名的， 存证拥有人是交易发送方，只有拥有人才可以吊销存证

  			// 判断存储单元里面是存在这样一个存证；如果不存在，抛出错误，错误我们叫ClaimNotExist
			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ClaimNotExist);

			// 获取这样的存证  owner: accountId   block_number
			let (owner,_block_number) = Proofs::<T>::get(&claim);  // 通过get api获取这样的一个存证

			ensure!(owner == sender,Error::<T>::NotClaimOwner);  // 确保交易发送方是我们的存证人，如果不是，返回Error，这个Error我们叫NotClaimOwner

			// 以上校验完成之后，我们就可以删除我们的存证
		    // 存储向上调用remove函数进行删除
		    Proofs::<T>::remove(&claim);

			// 触发一个事件，返回存证人和hash
		    Self::deposit_event(RawEvent::ClaimRevoked(sender,claim));

			// 返回
			Ok(())
        }

        #[pallet::weight(0)]
        pub fn revoke_claim(origin,claim: Vec<u8>) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;  // 交易发送方式已签名的， 存证拥有人是交易发送方，只有拥有人才可以吊销存证

  			// 判断存储单元里面是存在这样一个存证；如果不存在，抛出错误，错误我们叫ClaimNotExist
			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ClaimNotExist);

			// 获取这样的存证  owner: accountId   block_number
			let (owner,_block_number) = Proofs::<T>::get(&claim);  // 通过get api获取这样的一个存证

			ensure!(owner == sender,Error::<T>::NotClaimOwner);  // 确保交易发送方是我们的存证人，如果不是，返回Error，这个Error我们叫NotClaimOwner

			// 以上校验完成之后，我们就可以删除我们的存证
		    // 存储向上调用remove函数进行删除
		    Proofs::<T>::remove(&claim);

			// 触发一个事件，返回存证人和hash
		    Self::deposit_event(RawEvent::ClaimRevoked(sender,claim));

			// 返回
			Ok(())
        }

        #[pallet::weight(0)]
        pub fn transfer_claim(origin, claim: Vec<u8>, recipient: AccountId) -> DispatchResult {
            // 首先去创建签名交易，通过ensure_signed这样的system提供的版本方法来校验
            let sender = ensure_signed(origin)?;
        
            // 确保存证存在
            let (old_owner, _) = ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotFound);
            // 确保存证的拥有人是当前的交易发送者
            ensure!(old_owner == sender, Error::<T>::NotOwner);
        
            // 将存证的所有权转移到接收者
            Proofs::<T>::insert(&claim, (recipient, system::Module::<T>::block_number()));
        
            // 触发一个事件来通知客户端
            Self::deposit_event(RawEvent::ClaimTransferred(sender, recipient, claim));
        
            Ok(())
        }
    }
}
