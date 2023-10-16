/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodValueFrom : ValueFrom describes a location in which to obtain the value to a parameter



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodValueFrom {
    #[serde(rename = "configMapKeyRef", skip_serializing_if = "Option::is_none")]
    pub config_map_key_ref: Option<Box<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodConfigMapKeySelector>>,
    /// Default specifies a value to be used if retrieving the value from the specified source fails
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// Selector (https://github.com/antonmedv/expr) that is evaluated against the event to get the value of the parameter. E.g. `payload.message`
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// Expression, if defined, is evaluated to specify the value for the parameter
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// JQFilter expression against the resource object in resource templates
    #[serde(rename = "jqFilter", skip_serializing_if = "Option::is_none")]
    pub jq_filter: Option<String>,
    /// JSONPath of a resource to retrieve an output parameter value from in resource templates
    #[serde(rename = "jsonPath", skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    /// Parameter reference to a step or dag task in which to retrieve an output parameter value from (e.g. '{{steps.mystep.outputs.myparam}}')
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    /// Path in the container to retrieve an output parameter value from in container templates
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// SuppliedValueFrom is a placeholder for a value to be filled in directly, either through the CLI, API, etc.
    #[serde(rename = "supplied", skip_serializing_if = "Option::is_none")]
    pub supplied: Option<serde_json::Value>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodValueFrom {
    /// ValueFrom describes a location in which to obtain the value to a parameter
    pub fn new() -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodValueFrom {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodValueFrom {
            config_map_key_ref: None,
            default: None,
            event: None,
            expression: None,
            jq_filter: None,
            json_path: None,
            parameter: None,
            path: None,
            supplied: None,
        }
    }
}


