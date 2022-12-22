use frame_support::{
    dispatch::{self, DispatchResult},
    ensure,
};

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
