use crate::{
    pow::{self, HeaderHasher},
    proto::{
        anumad_message::Payload, GetBlockTemplateRequestMessage, GetInfoRequestMessage, AnumadMessage,
        NotifyBlockAddedRequestMessage, RpcBlock, SubmitBlockRequestMessage,
    },
    Hash,
};

impl AnumadMessage {
    #[must_use]
    #[inline(always)]
    pub fn get_info_request() -> Self {
        AnumadMessage { payload: Some(Payload::GetInfoRequest(GetInfoRequestMessage {})) }
    }
    #[must_use]
    #[inline(always)]
    pub fn notify_block_added() -> Self {
        AnumadMessage { payload: Some(Payload::NotifyBlockAddedRequest(NotifyBlockAddedRequestMessage {})) }
    }
    #[must_use]
    #[inline(always)]
    pub fn submit_block(block: RpcBlock) -> Self {
        AnumadMessage { payload: Some(Payload::SubmitBlockRequest(SubmitBlockRequestMessage { block: Some(block) })) }
    }
}

impl From<GetInfoRequestMessage> for AnumadMessage {
    #[inline(always)]
    fn from(a: GetInfoRequestMessage) -> Self {
        AnumadMessage { payload: Some(Payload::GetInfoRequest(a)) }
    }
}
impl From<NotifyBlockAddedRequestMessage> for AnumadMessage {
    #[inline(always)]
    fn from(a: NotifyBlockAddedRequestMessage) -> Self {
        AnumadMessage { payload: Some(Payload::NotifyBlockAddedRequest(a)) }
    }
}

impl From<GetBlockTemplateRequestMessage> for AnumadMessage {
    #[inline(always)]
    fn from(a: GetBlockTemplateRequestMessage) -> Self {
        AnumadMessage { payload: Some(Payload::GetBlockTemplateRequest(a)) }
    }
}

impl RpcBlock {
    #[must_use]
    #[inline(always)]
    pub fn block_hash(&self) -> Option<Hash> {
        let mut hasher = HeaderHasher::new();
        pow::serialize_header(&mut hasher, self.header.as_ref()?, false);
        Some(hasher.finalize())
    }
}
