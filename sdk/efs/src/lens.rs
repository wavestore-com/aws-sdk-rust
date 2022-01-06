// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_describe_access_points_output_next_token(
    input: &crate::output::DescribeAccessPointsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_file_systems_output_next_marker(
    input: &crate::output::DescribeFileSystemsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_tags_output_next_marker(
    input: &crate::output::DescribeTagsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_tags_for_resource_output_next_token(
    input: &crate::output::ListTagsForResourceOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
