//! # TPM Service Stub Library (Rust)
#![allow(dead_code, unused_imports, unused_variables)]

use crate::{Result, Service};
use odp_ffa::HasRegisterPayload;
use odp_ffa::{DirectMessagePayload, ErrorCode, MsgSendDirectReq2, MsgSendDirectResp2};
use uuid::{uuid, Uuid};

// ---------------------------------------------------------------------------
// TPM Service Implementation
// ---------------------------------------------------------------------------
pub struct TpmServiceStub {}

impl TpmServiceStub {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TpmServiceStub {
    fn default() -> Self {
        Self::new()
    }
}

impl Service for TpmServiceStub {
    const UUID: Uuid = uuid!("17b862a4-1806-4faf-86b3-089a58353861");
    const NAME: &'static str = "Tpm";

    fn ffa_msg_send_direct_req2(&mut self, msg: MsgSendDirectReq2) -> Result<MsgSendDirectResp2> {
        // x4-x17 are for payload (14 registers)
        let payload_regs = [ErrorCode::NotSupported as i64];
        let payload_bytes_iter = payload_regs.iter().flat_map(|&reg| i64::to_le_bytes(reg).into_iter());
        let resp_payload = DirectMessagePayload::from_iter(payload_bytes_iter);
        Ok(MsgSendDirectResp2::from_req_with_payload(&msg, resp_payload))
    }
}
