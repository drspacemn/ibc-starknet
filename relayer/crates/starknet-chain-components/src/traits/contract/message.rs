use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;

use crate::traits::types::address::HasAddressType;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasMethodSelectorType;

#[derive_component(InvokeContractMessageBuilderComponent, InvokeContractMessageBuilder<Chain>)]
pub trait CanBuildInvokeContractMessage:
    HasAddressType + HasMethodSelectorType + HasBlobType + HasMessageType
{
    fn build_invoke_contract_message(
        &self,
        contract_address: &Self::Address,
        selector: &Self::MethodSelector,
        call_data: &Self::Blob,
    ) -> Self::Message;
}
