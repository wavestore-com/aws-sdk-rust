// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_payload_create_component_input(
    payload: &std::option::Option<crate::model::CreateComponentData>,
) -> std::result::Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => {
            return Ok(aws_smithy_http::body::SdkBody::from(
                crate::operation_ser::rest_json_unsetpayload(),
            ))
        }
    };
    #[allow(clippy::useless_conversion)]Ok(aws_smithy_http::body::SdkBody::from(
        crate::operation_ser::serialize_member_com_amazonaws_amplifyuibuilder_synthetic_create_component_input_component_to_create(payload)?
    ))
}

pub fn ser_payload_create_theme_input(
    payload: &std::option::Option<crate::model::CreateThemeData>,
) -> std::result::Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => {
            return Ok(aws_smithy_http::body::SdkBody::from(
                crate::operation_ser::rest_json_unsetpayload(),
            ))
        }
    };
    #[allow(clippy::useless_conversion)]Ok(aws_smithy_http::body::SdkBody::from(
        crate::operation_ser::serialize_member_com_amazonaws_amplifyuibuilder_synthetic_create_theme_input_theme_to_create(payload)?
    ))
}

pub fn ser_payload_exchange_code_for_token_input(
    payload: &std::option::Option<crate::model::ExchangeCodeForTokenRequestBody>,
) -> std::result::Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => {
            return Ok(aws_smithy_http::body::SdkBody::from(
                crate::operation_ser::rest_json_unsetpayload(),
            ))
        }
    };
    #[allow(clippy::useless_conversion)]Ok(aws_smithy_http::body::SdkBody::from(
        crate::operation_ser::serialize_member_com_amazonaws_amplifyuibuilder_synthetic_exchange_code_for_token_input_request(payload)?
    ))
}

pub fn ser_payload_refresh_token_input(
    payload: &std::option::Option<crate::model::RefreshTokenRequestBody>,
) -> std::result::Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => {
            return Ok(aws_smithy_http::body::SdkBody::from(
                crate::operation_ser::rest_json_unsetpayload(),
            ))
        }
    };
    #[allow(clippy::useless_conversion)]Ok(aws_smithy_http::body::SdkBody::from(
        crate::operation_ser::serialize_member_com_amazonaws_amplifyuibuilder_synthetic_refresh_token_input_refresh_token_body(payload)?
    ))
}

pub fn ser_payload_update_component_input(
    payload: &std::option::Option<crate::model::UpdateComponentData>,
) -> std::result::Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => {
            return Ok(aws_smithy_http::body::SdkBody::from(
                crate::operation_ser::rest_json_unsetpayload(),
            ))
        }
    };
    #[allow(clippy::useless_conversion)]Ok(aws_smithy_http::body::SdkBody::from(
        crate::operation_ser::serialize_member_com_amazonaws_amplifyuibuilder_synthetic_update_component_input_updated_component(payload)?
    ))
}

pub fn ser_payload_update_theme_input(
    payload: &std::option::Option<crate::model::UpdateThemeData>,
) -> std::result::Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => {
            return Ok(aws_smithy_http::body::SdkBody::from(
                crate::operation_ser::rest_json_unsetpayload(),
            ))
        }
    };
    #[allow(clippy::useless_conversion)]Ok(aws_smithy_http::body::SdkBody::from(
        crate::operation_ser::serialize_member_com_amazonaws_amplifyuibuilder_synthetic_update_theme_input_updated_theme(payload)?
    ))
}

pub fn rest_json_unsetpayload() -> std::vec::Vec<u8> {
    b"{}"[..].into()
}

pub fn serialize_member_com_amazonaws_amplifyuibuilder_synthetic_create_component_input_component_to_create(
    input: &crate::model::CreateComponentData,
) -> std::result::Result<std::vec::Vec<u8>, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_model_create_component_data(&mut object, input)?;
    object.finish();
    Ok(out.into_bytes())
}

pub fn serialize_member_com_amazonaws_amplifyuibuilder_synthetic_create_theme_input_theme_to_create(
    input: &crate::model::CreateThemeData,
) -> std::result::Result<std::vec::Vec<u8>, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_model_create_theme_data(&mut object, input)?;
    object.finish();
    Ok(out.into_bytes())
}

pub fn serialize_member_com_amazonaws_amplifyuibuilder_synthetic_exchange_code_for_token_input_request(
    input: &crate::model::ExchangeCodeForTokenRequestBody,
) -> std::result::Result<std::vec::Vec<u8>, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_model_exchange_code_for_token_request_body(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(out.into_bytes())
}

pub fn serialize_member_com_amazonaws_amplifyuibuilder_synthetic_refresh_token_input_refresh_token_body(
    input: &crate::model::RefreshTokenRequestBody,
) -> std::result::Result<std::vec::Vec<u8>, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_model_refresh_token_request_body(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(out.into_bytes())
}

pub fn serialize_member_com_amazonaws_amplifyuibuilder_synthetic_update_component_input_updated_component(
    input: &crate::model::UpdateComponentData,
) -> std::result::Result<std::vec::Vec<u8>, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_model_update_component_data(&mut object, input)?;
    object.finish();
    Ok(out.into_bytes())
}

pub fn serialize_member_com_amazonaws_amplifyuibuilder_synthetic_update_theme_input_updated_theme(
    input: &crate::model::UpdateThemeData,
) -> std::result::Result<std::vec::Vec<u8>, aws_smithy_http::operation::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_model_update_theme_data(&mut object, input)?;
    object.finish();
    Ok(out.into_bytes())
}
