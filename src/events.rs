extern crate alloc;

use alloc::vec::Vec;
use ethereum_types::{H160, H256, U256};
use evm_runtime::ExitReason;
use parity_scale_codec::{Decode, Encode};
use evm_runtime::Context;

#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq)]
pub struct Transfer {
	/// Source address.
	pub source: H160,
	/// Target address.
	pub target: H160,
	/// Transfer value.
	pub value: U256,
}

impl From<evm_runtime::Transfer> for Transfer {
	fn from(i: evm_runtime::Transfer) -> Self {
		Self {
			source: i.source,
			target: i.target,
			value: i.value,
		}
	}
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Encode, Decode)]
pub enum CreateScheme {
	/// Legacy create scheme of `CREATE`.
	Legacy {
		/// Caller of the create.
		caller: H160,
	},
	/// Create scheme of `CREATE2`.
	Create2 {
		/// Caller of the create.
		caller: H160,
		/// Code hash.
		code_hash: H256,
		/// Salt.
		salt: H256,
	},
	/// Create at a fixed location.
	Fixed(H160),
}

impl From<evm_runtime::CreateScheme> for CreateScheme {
	fn from(i: evm_runtime::CreateScheme) -> Self {
		match i {
			evm_runtime::CreateScheme::Legacy { caller } => Self::Legacy { caller },
			evm_runtime::CreateScheme::Create2 {
				caller,
				code_hash,
				salt,
			} => Self::Create2 {
				caller,
				code_hash,
				salt,
			},
			evm_runtime::CreateScheme::Fixed(address) => Self::Fixed(address),
		}
	}
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub enum EvmEvent {
	Call {
		code_address: H160,
		transfer: Option<Transfer>,
		input: Vec<u8>,
		target_gas: Option<u64>,
		is_static: bool,
		context: Context,
	},
	Create {
		caller: H160,
		address: H160,
		scheme: CreateScheme,
		value: U256,
		init_code: Vec<u8>,
		target_gas: Option<u64>,
	},
	Suicide {
		address: H160,
		target: H160,
		balance: U256,
	},
	Exit {
		reason: ExitReason,
		return_value: Vec<u8>,
	},
	TransactCall {
		caller: H160,
		address: H160,
		value: U256,
		data: Vec<u8>,
		gas_limit: u64,
	},
	TransactCreate {
		caller: H160,
		value: U256,
		init_code: Vec<u8>,
		gas_limit: u64,
		address: H160,
	},
	TransactCreate2 {
		caller: H160,
		value: U256,
		init_code: Vec<u8>,
		salt: H256,
		gas_limit: u64,
		address: H160,
	},
	PrecompileSubcall {
		code_address: H160,
		transfer: Option<Transfer>,
		input: Vec<u8>,
		target_gas: Option<u64>,
		is_static: bool,
		context: Context,
	},
}

#[cfg(feature = "tracing")]
impl<'a> From<crate::tracing::Event<'a>> for EvmEvent {
	fn from(i: crate::tracing::Event<'a>) -> Self {
		match i {
			crate::tracing::Event::Call {
				code_address,
				transfer,
				input,
				target_gas,
				is_static,
				context,
			} => Self::Call {
				code_address,
				transfer: if let Some(transfer) = transfer {
					Some(transfer.clone().into())
				} else {
					None
				},
				input: input.to_vec(),
				target_gas,
				is_static,
				context: context.clone().into(),
			},
			crate::tracing::Event::Create {
				caller,
				address,
				scheme,
				value,
				init_code,
				target_gas,
			} => Self::Create {
				caller,
				address,
				scheme: scheme.into(),
				value,
				init_code: init_code.to_vec(),
				target_gas,
			},
			crate::tracing::Event::Suicide {
				address,
				target,
				balance,
			} => Self::Suicide {
				address,
				target,
				balance,
			},
			crate::tracing::Event::Exit {
				reason,
				return_value,
			} => Self::Exit {
				reason: reason.clone(),
				return_value: return_value.to_vec(),
			},
			crate::tracing::Event::TransactCall {
				caller,
				address,
				value,
				data,
				gas_limit,
			} => Self::TransactCall {
				caller,
				address,
				value,
				data: data.to_vec(),
				gas_limit,
			},
			crate::tracing::Event::TransactCreate {
				caller,
				value,
				init_code,
				gas_limit,
				address,
			} => Self::TransactCreate {
				caller,
				value,
				init_code: init_code.to_vec(),
				gas_limit,
				address,
			},
			crate::tracing::Event::TransactCreate2 {
				caller,
				value,
				init_code,
				salt,
				gas_limit,
				address,
			} => Self::TransactCreate2 {
				caller,
				value,
				init_code: init_code.to_vec(),
				salt,
				gas_limit,
				address,
			},
			crate::tracing::Event::PrecompileSubcall {
				code_address,
				transfer,
				input,
				target_gas,
				is_static,
				context,
			} => Self::PrecompileSubcall {
				code_address,
				transfer: if let Some(transfer) = transfer {
					Some(transfer.clone().into())
				} else {
					None
				},
				input: input.to_vec(),
				target_gas,
				is_static,
				context: context.clone().into(),
			},
		}
	}
}
