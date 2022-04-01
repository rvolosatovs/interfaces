// This file is generated automatically using wasmcloud/weld-codegen 0.4.3

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

pub type Dimensions = Vec<u32>;

// Encode Dimensions as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_dimensions<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Dimensions,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        e.u32(*item)?;
    }
    Ok(())
}

// Decode Dimensions from cbor input stream
#[doc(hidden)]
pub fn decode_dimensions(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Dimensions, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<u32> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(d.u32()?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<u32> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(d.u32()?),
                }
            }
            arr
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct InferenceInput {
    /// Name of model to invoke
    #[serde(default)]
    pub model: String,
    /// Tensor data
    pub tensor: Tensor,
    ///
    #[serde(default)]
    pub index: u32,
}

// Encode InferenceInput as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_inference_input<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &InferenceInput,
) -> RpcResult<()> {
    e.array(3)?;
    e.str(&val.model)?;
    encode_tensor(e, &val.tensor)?;
    e.u32(val.index)?;
    Ok(())
}

// Decode InferenceInput from cbor input stream
#[doc(hidden)]
pub fn decode_inference_input(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<InferenceInput, RpcError> {
    let __result = {
        let mut model: Option<String> = None;
        let mut tensor: Option<Tensor> = None;
        let mut index: Option<u32> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct InferenceInput, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => model = Some(d.str()?.to_string()),
                    1 => {
                        tensor = Some(decode_tensor(d).map_err(|e| {
                            format!(
                                "decoding 'org.wasmcloud.interface.mlinference#Tensor': {}",
                                e
                            )
                        })?)
                    }
                    2 => index = Some(d.u32()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "model" => model = Some(d.str()?.to_string()),
                    "tensor" => {
                        tensor = Some(decode_tensor(d).map_err(|e| {
                            format!(
                                "decoding 'org.wasmcloud.interface.mlinference#Tensor': {}",
                                e
                            )
                        })?)
                    }
                    "index" => index = Some(d.u32()?),
                    _ => d.skip()?,
                }
            }
        }
        InferenceInput {
            model: if let Some(__x) = model {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field InferenceInput.model (#0)".to_string(),
                ));
            },

            tensor: if let Some(__x) = tensor {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field InferenceInput.tensor (#1)".to_string(),
                ));
            },

            index: if let Some(__x) = index {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field InferenceInput.index (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// InferenceOutput
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InferenceOutput {
    pub result: Status,
    pub tensor: Tensor,
}

// Encode InferenceOutput as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_inference_output<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &InferenceOutput,
) -> RpcResult<()> {
    e.array(2)?;
    encode_status(e, &val.result)?;
    encode_tensor(e, &val.tensor)?;
    Ok(())
}

// Decode InferenceOutput from cbor input stream
#[doc(hidden)]
pub fn decode_inference_output(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<InferenceOutput, RpcError> {
    let __result = {
        let mut result: Option<Status> = None;
        let mut tensor: Option<Tensor> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct InferenceOutput, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        result = Some(decode_status(d).map_err(|e| {
                            format!(
                                "decoding 'org.wasmcloud.interface.mlinference#Status': {}",
                                e
                            )
                        })?)
                    }
                    1 => {
                        tensor = Some(decode_tensor(d).map_err(|e| {
                            format!(
                                "decoding 'org.wasmcloud.interface.mlinference#Tensor': {}",
                                e
                            )
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "result" => {
                        result = Some(decode_status(d).map_err(|e| {
                            format!(
                                "decoding 'org.wasmcloud.interface.mlinference#Status': {}",
                                e
                            )
                        })?)
                    }
                    "tensor" => {
                        tensor = Some(decode_tensor(d).map_err(|e| {
                            format!(
                                "decoding 'org.wasmcloud.interface.mlinference#Tensor': {}",
                                e
                            )
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        }
        InferenceOutput {
            result: if let Some(__x) = result {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field InferenceOutput.result (#0)".to_string(),
                ));
            },

            tensor: if let Some(__x) = tensor {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field InferenceOutput.tensor (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Error returned with InferenceOutput
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MlError {
    /// n(0)
    InvalidModel(String),
    /// n(1)
    InvalidEncoding(String),
    /// n(2)
    CorruptInputTensor(String),
    /// n(3)
    RuntimeError(String),
    /// n(4)
    OpenVinoError(String),
    /// n(5)
    OnnxError(String),
    /// n(6)
    TensorflowError(String),
    /// n(7)
    ContextNotFoundError(String),
}

// Encode MlError as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_ml_error<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &MlError,
) -> RpcResult<()> {
    // encoding union MlError
    e.array(2)?;
    match val {
        MlError::InvalidModel(v) => {
            e.u16(0)?;
            e.str(v)?;
        }
        MlError::InvalidEncoding(v) => {
            e.u16(1)?;
            e.str(v)?;
        }
        MlError::CorruptInputTensor(v) => {
            e.u16(2)?;
            e.str(v)?;
        }
        MlError::RuntimeError(v) => {
            e.u16(3)?;
            e.str(v)?;
        }
        MlError::OpenVinoError(v) => {
            e.u16(4)?;
            e.str(v)?;
        }
        MlError::OnnxError(v) => {
            e.u16(5)?;
            e.str(v)?;
        }
        MlError::TensorflowError(v) => {
            e.u16(6)?;
            e.str(v)?;
        }
        MlError::ContextNotFoundError(v) => {
            e.u16(7)?;
            e.str(v)?;
        }
    }
    Ok(())
}

// Decode MlError from cbor input stream
#[doc(hidden)]
pub fn decode_ml_error(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<MlError, RpcError> {
    let __result = {
        // decoding union MlError
        let len = d.fixed_array()?;
        if len != 2 {
            return Err(RpcError::Deser(
                "decoding union 'MlError': expected 2-array".to_string(),
            ));
        }
        match d.u16()? {
            0 => {
                let val = d.str()?.to_string();
                MlError::InvalidModel(val)
            }

            1 => {
                let val = d.str()?.to_string();
                MlError::InvalidEncoding(val)
            }

            2 => {
                let val = d.str()?.to_string();
                MlError::CorruptInputTensor(val)
            }

            3 => {
                let val = d.str()?.to_string();
                MlError::RuntimeError(val)
            }

            4 => {
                let val = d.str()?.to_string();
                MlError::OpenVinoError(val)
            }

            5 => {
                let val = d.str()?.to_string();
                MlError::OnnxError(val)
            }

            6 => {
                let val = d.str()?.to_string();
                MlError::TensorflowError(val)
            }

            7 => {
                let val = d.str()?.to_string();
                MlError::ContextNotFoundError(val)
            }

            n => {
                return Err(RpcError::Deser(format!(
                    "invalid field number for union \
                     'org.wasmcloud.interface.mlinference#MlError':{}",
                    n
                )));
            }
        }
    };
    Ok(__result)
}
/// Response is either success or an error code
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
    /// n(0)
    Success,
    /// n(1)
    Error(MlError),
}

// Encode Status as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_status<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Status,
) -> RpcResult<()> {
    // encoding union Status
    e.array(2)?;
    match val {
        Status::Success => {
            e.u16(0)?;
            e.null()?;
        }
        Status::Error(v) => {
            e.u16(1)?;
            encode_ml_error(e, v)?;
        }
    }
    Ok(())
}

// Decode Status from cbor input stream
#[doc(hidden)]
pub fn decode_status(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Status, RpcError> {
    let __result = {
        // decoding union Status
        let len = d.fixed_array()?;
        if len != 2 {
            return Err(RpcError::Deser(
                "decoding union 'Status': expected 2-array".to_string(),
            ));
        }
        match d.u16()? {
            0 => {
                d.null()?;
                Status::Success
            }

            1 => {
                let val = decode_ml_error(d).map_err(|e| {
                    format!(
                        "decoding 'org.wasmcloud.interface.mlinference#MlError': {}",
                        e
                    )
                })?;
                Status::Error(val)
            }

            n => {
                return Err(RpcError::Deser(format!(
                    "invalid field number for union \
                     'org.wasmcloud.interface.mlinference#Status':{}",
                    n
                )));
            }
        }
    };
    Ok(__result)
}
/// The tensor's dimensions and type are provided as metadata to a model.
/// Any metadata shall be associated to the respective model in a blob store.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Tensor {
    /// Tensor Dimensions
    /// The Dimension array contains one size value for each dimension
    /// of the Tensor
    pub dimensions: Dimensions,
    /// The types array contains either: a single ValueType
    /// that represents the data values for all dimensions (homogeneous array)
    /// or one ValueType per dimension. In other words, the length
    /// of this array is either 1 or the length of `dimensions`.
    #[serde(rename = "valueTypes")]
    pub value_types: ValueTypes,
    /// Optional bit flags representing the data representation in the Tensor.
    /// Currently only one bit (LSB) is used to indicate
    /// row-major order (0) or column-major order (1).
    #[serde(default)]
    pub flags: u8,
    /// The Tensor
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub data: Vec<u8>,
}

// Encode Tensor as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_tensor<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Tensor,
) -> RpcResult<()> {
    e.array(4)?;
    encode_dimensions(e, &val.dimensions)?;
    encode_value_types(e, &val.value_types)?;
    e.u8(val.flags)?;
    e.bytes(&val.data)?;
    Ok(())
}

// Decode Tensor from cbor input stream
#[doc(hidden)]
pub fn decode_tensor(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Tensor, RpcError> {
    let __result = {
        let mut dimensions: Option<Dimensions> = None;
        let mut value_types: Option<ValueTypes> = None;
        let mut flags: Option<u8> = None;
        let mut data: Option<Vec<u8>> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Tensor, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        dimensions = Some(decode_dimensions(d).map_err(|e| {
                            format!(
                                "decoding 'org.wasmcloud.interface.mlinference#Dimensions': {}",
                                e
                            )
                        })?)
                    }
                    1 => {
                        value_types = Some(decode_value_types(d).map_err(|e| {
                            format!(
                                "decoding 'org.wasmcloud.interface.mlinference#ValueTypes': {}",
                                e
                            )
                        })?)
                    }
                    2 => flags = Some(d.u8()?),
                    3 => data = Some(d.bytes()?.to_vec()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "dimensions" => {
                        dimensions = Some(decode_dimensions(d).map_err(|e| {
                            format!(
                                "decoding 'org.wasmcloud.interface.mlinference#Dimensions': {}",
                                e
                            )
                        })?)
                    }
                    "valueTypes" => {
                        value_types = Some(decode_value_types(d).map_err(|e| {
                            format!(
                                "decoding 'org.wasmcloud.interface.mlinference#ValueTypes': {}",
                                e
                            )
                        })?)
                    }
                    "flags" => flags = Some(d.u8()?),
                    "data" => data = Some(d.bytes()?.to_vec()),
                    _ => d.skip()?,
                }
            }
        }
        Tensor {
            dimensions: if let Some(__x) = dimensions {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Tensor.dimensions (#0)".to_string(),
                ));
            },

            value_types: if let Some(__x) = value_types {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Tensor.value_types (#1)".to_string(),
                ));
            },

            flags: if let Some(__x) = flags {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Tensor.flags (#2)".to_string(),
                ));
            },

            data: if let Some(__x) = data {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Tensor.data (#3)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Value of a data element in a tensor
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ValueType {
    /// Unsigned 8-bit data (0x00) (b0000 0000)
    /// n(0)
    ValueU8,
    /// Unsigned 16-bit data (0x01) (b0000 0001)
    /// n(1)
    ValueU16,
    /// Unsigned 32-bit data (0x02) (b0000 0010)
    /// n(2)
    ValueU32,
    /// Unsigned 64-bit data (0x03) (b0000 0011)
    /// n(3)
    ValueU64,
    /// Unsigned 128-bit data (0x04) (b0000 0100)
    /// n(4)
    ValueU128,
    /// Signed 8-bit data (0x40) (b0100 0000)
    /// n(64)
    ValueS8,
    /// Signed 16-bit data (0x41) (b0100 0001)
    /// n(65)
    ValueS16,
    /// Signed 32-bit data (0x42) (b0100 0010)
    /// n(66)
    ValueS32,
    /// Signed 64-bit data (0x43) (b0100 0011)
    /// n(67)
    ValueS64,
    /// Signed 128-bit data (0x44) (b0100 0100)
    /// n(68)
    ValueS128,
    /// 16-bit IEEE Float (0x81) (b1000 0001)
    /// n(129)
    ValueF16,
    /// 32-bit IEEE Float (0x82) (b1000 0010)
    /// n(130)
    ValueF32,
    /// 64-bit IEEE Float (0x83) (b1000 0011)
    /// n(131)
    ValueF64,
    /// 128-bit IEEE Float (0x84) (b1000 0100)
    /// n(132)
    ValueF128,
}

// Encode ValueType as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_value_type<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ValueType,
) -> RpcResult<()> {
    // encoding union ValueType
    e.array(2)?;
    match val {
        ValueType::ValueU8 => {
            e.u16(0)?;
            e.null()?;
        }
        ValueType::ValueU16 => {
            e.u16(1)?;
            e.null()?;
        }
        ValueType::ValueU32 => {
            e.u16(2)?;
            e.null()?;
        }
        ValueType::ValueU64 => {
            e.u16(3)?;
            e.null()?;
        }
        ValueType::ValueU128 => {
            e.u16(4)?;
            e.null()?;
        }
        ValueType::ValueS8 => {
            e.u16(64)?;
            e.null()?;
        }
        ValueType::ValueS16 => {
            e.u16(65)?;
            e.null()?;
        }
        ValueType::ValueS32 => {
            e.u16(66)?;
            e.null()?;
        }
        ValueType::ValueS64 => {
            e.u16(67)?;
            e.null()?;
        }
        ValueType::ValueS128 => {
            e.u16(68)?;
            e.null()?;
        }
        ValueType::ValueF16 => {
            e.u16(129)?;
            e.null()?;
        }
        ValueType::ValueF32 => {
            e.u16(130)?;
            e.null()?;
        }
        ValueType::ValueF64 => {
            e.u16(131)?;
            e.null()?;
        }
        ValueType::ValueF128 => {
            e.u16(132)?;
            e.null()?;
        }
    }
    Ok(())
}

// Decode ValueType from cbor input stream
#[doc(hidden)]
pub fn decode_value_type(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<ValueType, RpcError> {
    let __result = {
        // decoding union ValueType
        let len = d.fixed_array()?;
        if len != 2 {
            return Err(RpcError::Deser(
                "decoding union 'ValueType': expected 2-array".to_string(),
            ));
        }
        match d.u16()? {
            0 => {
                d.null()?;
                ValueType::ValueU8
            }

            1 => {
                d.null()?;
                ValueType::ValueU16
            }

            2 => {
                d.null()?;
                ValueType::ValueU32
            }

            3 => {
                d.null()?;
                ValueType::ValueU64
            }

            4 => {
                d.null()?;
                ValueType::ValueU128
            }

            64 => {
                d.null()?;
                ValueType::ValueS8
            }

            65 => {
                d.null()?;
                ValueType::ValueS16
            }

            66 => {
                d.null()?;
                ValueType::ValueS32
            }

            67 => {
                d.null()?;
                ValueType::ValueS64
            }

            68 => {
                d.null()?;
                ValueType::ValueS128
            }

            129 => {
                d.null()?;
                ValueType::ValueF16
            }

            130 => {
                d.null()?;
                ValueType::ValueF32
            }

            131 => {
                d.null()?;
                ValueType::ValueF64
            }

            132 => {
                d.null()?;
                ValueType::ValueF128
            }

            n => {
                return Err(RpcError::Deser(format!(
                    "invalid field number for union \
                     'org.wasmcloud.interface.mlinference#ValueType':{}",
                    n
                )));
            }
        }
    };
    Ok(__result)
}
pub type ValueTypes = Vec<ValueType>;

// Encode ValueTypes as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_value_types<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ValueTypes,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_value_type(e, item)?;
    }
    Ok(())
}

// Decode ValueTypes from cbor input stream
#[doc(hidden)]
pub fn decode_value_types(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<ValueTypes, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<ValueType> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_value_type(d).map_err(|e| {
                    format!(
                        "decoding 'org.wasmcloud.interface.mlinference#ValueType': {}",
                        e
                    )
                })?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<ValueType> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(decode_value_type(d).map_err(|e| {
                        format!(
                            "decoding 'org.wasmcloud.interface.mlinference#ValueType': {}",
                            e
                        )
                    })?),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// The MlInference service
/// wasmbus.contractId: wasmcloud:mlinference
/// wasmbus.providerReceive
/// wasmbus.actorReceive
#[async_trait]
pub trait MlInference {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:mlinference"
    }
    /// predict
    async fn predict(&self, ctx: &Context, arg: &InferenceInput) -> RpcResult<InferenceOutput>;
}

/// MlInferenceReceiver receives messages defined in the MlInference service trait
/// The MlInference service
#[doc(hidden)]
#[async_trait]
pub trait MlInferenceReceiver: MessageDispatch + MlInference {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "Predict" => {
                let value: InferenceInput =
                    wasmbus_rpc::common::decode(&message.arg, &decode_inference_input)
                        .map_err(|e| RpcError::Deser(format!("'InferenceInput': {}", e)))?;
                let resp = MlInference::predict(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_inference_output(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "MlInference.Predict",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "MlInference::{}",
                message.method
            ))),
        }
    }
}

/// MlInferenceSender sends messages to a MlInference service
/// The MlInference service
/// client for sending MlInference messages
#[derive(Debug)]
pub struct MlInferenceSender<T: Transport> {
    transport: T,
}

impl<T: Transport> MlInferenceSender<T> {
    /// Constructs a MlInferenceSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> MlInferenceSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl MlInferenceSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}

#[cfg(target_arch = "wasm32")]
impl MlInferenceSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a MlInference provider
    /// implementing the 'wasmcloud:mlinference' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("wasmcloud:mlinference", "default")
                .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a MlInference provider
    /// implementing the 'wasmcloud:mlinference' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("wasmcloud:mlinference", link_name)?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> MlInference for MlInferenceSender<T> {
    #[allow(unused)]
    /// predict
    async fn predict(&self, ctx: &Context, arg: &InferenceInput) -> RpcResult<InferenceOutput> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_inference_input(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "MlInference.Predict",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: InferenceOutput =
            wasmbus_rpc::common::decode(&resp, &decode_inference_output)
                .map_err(|e| RpcError::Deser(format!("'{}': InferenceOutput", e)))?;
        Ok(value)
    }
}
