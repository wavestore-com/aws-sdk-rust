// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_connectors_output_next_token(
    input: &crate::output::ListConnectorsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_custom_plugins_output_next_token(
    input: &crate::output::ListCustomPluginsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_worker_configurations_output_next_token(
    input: &crate::output::ListWorkerConfigurationsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_connectors_output_connectors(
    input: crate::output::ListConnectorsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ConnectorSummary>> {
    let input = match input.connectors {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_custom_plugins_output_custom_plugins(
    input: crate::output::ListCustomPluginsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::CustomPluginSummary>> {
    let input = match input.custom_plugins {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_worker_configurations_output_worker_configurations(
    input: crate::output::ListWorkerConfigurationsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::WorkerConfigurationSummary>> {
    let input = match input.worker_configurations {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
