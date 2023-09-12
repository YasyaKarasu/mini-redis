#![feature(impl_trait_in_assoc_type)]

pub struct TimedValue {
	value: String,
	expire_at: Option<u128>,
}

impl std::fmt::Debug for TimedValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TimedValue")
			.field("value", &self.value)
			.field("expire_at", &self.expire_at)
			.finish()
	}
}

pub struct S {
	pub map: std::sync::Mutex<std::collections::HashMap<String, TimedValue>>,
}

#[volo::async_trait]
impl volo_gen::mini::redis::MiniRedisService for S {
	async fn get_value(&self, _request: volo_gen::mini::redis::GetValueRequest) -> ::core::result::Result<volo_gen::mini::redis::GetValueResponse, ::volo_thrift::AnyhowError>{
		let mut map = self.map.lock().unwrap();
		if let Some(value) = map.get(_request.key.as_str()) {
			if let Some(expire_at) = value.expire_at {
				if expire_at < std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis() {
					map.remove(_request.key.as_str());
					return Err(anyhow::anyhow!("Key Not Found").into());
				}
			}
			Ok(volo_gen::mini::redis::GetValueResponse { value: value.value.clone().into(), error: "OK".into()})
		} else {
			Err(anyhow::anyhow!("Key Not Found").into())
		}
	}
	async fn set_value(&self, _request: volo_gen::mini::redis::SetValueRequest) -> ::core::result::Result<volo_gen::mini::redis::SetValueResponse, ::volo_thrift::AnyhowError>{
		let expire_seconds = _request.expire_seconds.unwrap_or(0);
		if expire_seconds > 0 {
			self.map.lock().unwrap().insert(_request.key.clone().to_string(), TimedValue {
				value: _request.value.clone().to_string(),
				expire_at: Some(expire_seconds as u128 * 1000 + std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis()),
			});
		} else {
				self.map.lock().unwrap().insert(_request.key.clone().to_string(), TimedValue {
				value: _request.value.clone().to_string(),
				expire_at: None,
			});
		}
		Ok(volo_gen::mini::redis::SetValueResponse { error: "OK".into() })
	}
	async fn delete_value(&self, _request: volo_gen::mini::redis::DeleteValueRequest) -> ::core::result::Result<volo_gen::mini::redis::DeleteValueResponse, ::volo_thrift::AnyhowError>{
		if let Some(_) = self.map.lock().unwrap().remove(_request.key.as_str()) {
			Ok(volo_gen::mini::redis::DeleteValueResponse { error: "OK".into() })
		} else {
			Err(anyhow::anyhow!("Key Not Found").into())
		}
	}
	async fn ping(&self) -> ::core::result::Result<volo_gen::mini::redis::PingResponse, ::volo_thrift::AnyhowError>{
		Ok(volo_gen::mini::redis::PingResponse { pong: "Pong".into() })
	}
}
