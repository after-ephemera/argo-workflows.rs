/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodParameter : Parameter indicate a passed string parameter to a service template with an optional default value



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodParameter {
    /// Default is the default value to use for an input parameter if a value was not supplied
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// Description is the parameter description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Enum holds a list of string values to choose from, for the actual value of the parameter
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
    /// GlobalName exports an output parameter to the global scope, making it available as '{{io.argoproj.workflow.v1alpha1.outputs.parameters.XXXX}} and in workflow.status.outputs.parameters
    #[serde(rename = "globalName", skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
    /// Name is the parameter name
    #[serde(rename = "name")]
    pub name: String,
    /// Value is the literal value to use for the parameter. If specified in the context of an input parameter, the value takes precedence over any passed values
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "valueFrom", skip_serializing_if = "Option::is_none")]
    pub value_from: Option<Box<crate::models::IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodValueFrom>>,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodParameter {
    /// Parameter indicate a passed string parameter to a service template with an optional default value
    pub fn new(name: String) -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodParameter {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodParameter {
            default: None,
            description: None,
            r#enum: None,
            global_name: None,
            name,
            value: None,
            value_from: None,
        }
    }
}


