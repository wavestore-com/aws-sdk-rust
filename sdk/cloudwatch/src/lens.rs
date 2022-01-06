// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_describe_alarm_history_output_next_token(
    input: &crate::output::DescribeAlarmHistoryOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_alarms_output_next_token(
    input: &crate::output::DescribeAlarmsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_insight_rules_output_next_token(
    input: &crate::output::DescribeInsightRulesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_get_metric_data_output_next_token(
    input: &crate::output::GetMetricDataOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_dashboards_output_next_token(
    input: &crate::output::ListDashboardsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_metrics_output_next_token(
    input: &crate::output::ListMetricsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_metric_streams_output_next_token(
    input: &crate::output::ListMetricStreamsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_alarm_history_output_alarm_history_items(
    input: crate::output::DescribeAlarmHistoryOutput,
) -> std::option::Option<std::vec::Vec<crate::model::AlarmHistoryItem>> {
    let input = match input.alarm_history_items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_dashboards_output_dashboard_entries(
    input: crate::output::ListDashboardsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::DashboardEntry>> {
    let input = match input.dashboard_entries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_metrics_output_metrics(
    input: crate::output::ListMetricsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Metric>> {
    let input = match input.metrics {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
