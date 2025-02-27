use cgp::prelude::*;
use hermes_encoding_components::impls::convert::{ConvertFrom, TryConvertFrom};
use hermes_encoding_components::impls::convert_and_encode::ConvertAndEncode;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::return_encoded::ReturnEncoded;
use hermes_encoding_components::impls::schema::ProvideStringSchema;
pub use hermes_encoding_components::traits::convert::ConverterComponent;
pub use hermes_encoding_components::traits::decode::DecoderComponent;
pub use hermes_encoding_components::traits::encode::EncoderComponent;
pub use hermes_encoding_components::traits::schema::SchemaGetterComponent;
pub use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;
use hermes_protobuf_encoding_components::impl_type_url;
use hermes_protobuf_encoding_components::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};
use hermes_protobuf_encoding_components::impls::from_context::EncodeFromContext;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::{Any, ViaAny, ViaProtobuf};
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;
use hermes_wasm_client_components::types::client_state::{ProtoWasmClientState, WasmClientState};
use hermes_wasm_client_components::types::consensus_state::{
    ProtoWasmConsensusState, WasmConsensusState,
};
use ibc::clients::wasm_types::client_message::ClientMessage;
use ibc_proto::ibc::lightclients::wasm::v1::ClientMessage as ProtoClientMessage;

use crate::types::client_header::{ConvertStarknetClientHeader, StarknetClientHeader};
use crate::types::client_state::{
    ConvertWasmStarknetClientState, ProtoStarknetClientState, StarknetClientState,
    WasmStarknetClientState,
};
use crate::types::consensus_state::{
    ConvertWasmStarknetConsensusState, ProtoStarknetConsensusState, StarknetConsensusState,
    WasmStarknetConsensusState,
};

define_components! {
    StarknetProtobufEncodingComponents {
        EncodedTypeComponent:
            ProvideEncodedBytes,
        SchemaTypeComponent:
            ProvideStringSchema,
        ConverterComponent:
            DelegateEncoding<StarknetConverterComponents>,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            DelegateEncoding<StarknetEncoderComponents>,
        SchemaGetterComponent:
            DelegateEncoding<StarknetTypeUrlSchemas>,
    }
}

pub struct StarknetEncoderComponents;

pub struct StarknetConverterComponents;

pub struct StarknetTypeUrlSchemas;

delegate_components! {
    StarknetEncoderComponents {
        (ViaProtobuf, Vec<u8>): ReturnEncoded,

        (ViaAny, StarknetClientState): EncodeViaAny<ViaProtobuf>,

        (ViaProtobuf, StarknetClientState): ConvertAndEncode<ProtoStarknetClientState>,
        (ViaProtobuf, ProtoStarknetClientState): EncodeAsProtobuf,

        (ViaAny, StarknetConsensusState): EncodeViaAny<ViaProtobuf>,

        (ViaProtobuf, StarknetConsensusState): ConvertAndEncode<ProtoStarknetConsensusState>,
        (ViaProtobuf, ProtoStarknetConsensusState): EncodeAsProtobuf,

        (ViaAny, ClientMessage): EncodeViaAny<ViaProtobuf>,

        (ViaProtobuf, ClientMessage): ConvertAndEncode<ProtoClientMessage>,
        (ViaProtobuf, ProtoClientMessage): EncodeAsProtobuf,

        (ViaProtobuf, Any): EncodeAsProtobuf,

        [
            (ViaAny, WasmClientState),
            (ViaProtobuf, WasmClientState),
            (ViaProtobuf, ProtoWasmClientState),
            (ViaAny, WasmConsensusState),
            (ViaProtobuf, WasmConsensusState),
            (ViaProtobuf, ProtoWasmConsensusState),
        ]:
            WasmEncodingComponents,
    }
}

delegate_components! {
    StarknetConverterComponents {
        (StarknetClientState, ProtoStarknetClientState): ConvertFrom,
        (ProtoStarknetClientState, StarknetClientState): TryConvertFrom,

        (StarknetConsensusState, ProtoStarknetConsensusState): ConvertFrom,
        (ProtoStarknetConsensusState, StarknetConsensusState): TryConvertFrom,

        (ClientMessage, ProtoClientMessage): ConvertFrom,
        (ProtoClientMessage, ClientMessage): TryConvertFrom,

        (StarknetClientState, Any): EncodeAsAnyProtobuf<ViaProtobuf, EncodeFromContext>,
        (Any, StarknetClientState): DecodeAsAnyProtobuf<ViaProtobuf, EncodeFromContext>,

        (StarknetConsensusState, Any): EncodeAsAnyProtobuf<ViaProtobuf, EncodeFromContext>,
        (Any, StarknetConsensusState): DecodeAsAnyProtobuf<ViaProtobuf, EncodeFromContext>,

        (ClientMessage, Any): EncodeAsAnyProtobuf<ViaProtobuf, EncodeFromContext>,
        (Any, ClientMessage): DecodeAsAnyProtobuf<ViaProtobuf, EncodeFromContext>,

        [
            (WasmClientState, ProtoWasmClientState),
            (ProtoWasmClientState, WasmClientState),
            (WasmConsensusState, ProtoWasmConsensusState),
            (ProtoWasmConsensusState, WasmConsensusState),
            (WasmClientState, Any),
            (Any, WasmClientState),
            (WasmConsensusState, Any),
            (Any, WasmConsensusState),
        ]:
            WasmEncodingComponents,

        [
            (Any, WasmStarknetClientState),
            (WasmStarknetClientState, Any),
        ]:
            ConvertWasmStarknetClientState,

        [
            (Any, WasmStarknetConsensusState),
            (WasmStarknetConsensusState, Any),
        ]:
            ConvertWasmStarknetConsensusState,

        [
            (Any, StarknetClientHeader),
            (StarknetClientHeader, Any),
        ]:
            ConvertStarknetClientHeader,

    }
}

delegate_components! {
    StarknetTypeUrlSchemas {
        StarknetClientState: StarknetClientStateUrl,
        StarknetConsensusState: StarknetConsensusStateUrl,
        ClientMessage: ClientMessageUrl,
        [
            WasmClientState,
            WasmConsensusState,
        ]:
            WasmEncodingComponents,
    }
}

impl_type_url!(StarknetClientStateUrl, "/StarknetClientState");

impl_type_url!(StarknetConsensusStateUrl, "/StarknetConsensusState");

impl_type_url!(ClientMessageUrl, "/ibc.lightclients.wasm.v1.ClientMessage");
