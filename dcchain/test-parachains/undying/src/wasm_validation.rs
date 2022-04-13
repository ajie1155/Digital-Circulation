// Copyright 2022 Parity Technologies (UK) Ltd.
// This file is part of Digital Circulation.

// Digital Circulation is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Digital Circulation is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Digital Circulation.  If not.

//! WASM validation for the `Undying` dcchain.

use crate::{BlockData, HeadData};
use dcchain::primitives::{HeadData as GenericHeadData, ValidationResult};
use parity_scale_codec::{Decode, Encode};

#[no_mangle]
pub extern "C" fn validate_block(params: *const u8, len: usize) -> u64 {
	let params = unsafe { dcchain::load_params(params, len) };
	let parent_head =
		HeadData::decode(&mut &params.parent_head.0[..]).expect("invalid parent head format.");

	let mut block_data =
		BlockData::decode(&mut &params.block_data.0[..]).expect("invalid block data format.");

	let parent_hash = crate::keccak256(&params.parent_head.0[..]);

	let (new_head, _) =
		crate::execute(parent_hash, parent_head, block_data).expect("Executes block");

	dcchain::write_result(&ValidationResult {
		head_data: GenericHeadData(new_head.encode()),
		new_validation_code: None,
		upward_messages: sp_std::vec::Vec::new(),
		horizontal_messages: sp_std::vec::Vec::new(),
		processed_downward_messages: 0,
		hrmp_watermark: params.relay_parent_number,
	})
}