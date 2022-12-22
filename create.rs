// 创建存证，创建存证需要有两个关键参数：交易发送方origin，存证hash值claim，由于存证hash函数未知，也和decl_storage定义对应，这里使用变长Vec<u8>
#[weight = 0]
		pub fn create_claim(origin,claim:Vec<u8>)->dispatch::DispatchResult{
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
