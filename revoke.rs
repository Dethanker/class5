#[weight = 0]
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
