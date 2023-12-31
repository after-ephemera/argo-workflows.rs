/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTriggerParameter {
    /// Dest is the JSONPath of a resource key. A path is a series of keys separated by a dot. The colon character can be escaped with '.' The -1 key can be used to append a value to an existing array. See https://github.com/tidwall/sjson#path-syntax for more information about how this is used.
    #[serde(rename = "dest", skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,
    /// Operation is what to do with the existing value at Dest, whether to 'prepend', 'overwrite', or 'append' it.
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "src", skip_serializing_if = "Option::is_none")]
    pub src: Option<Box<crate::models::IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTriggerParameterSource>>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTriggerParameter {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTriggerParameter {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodTriggerParameter {
            dest: None,
            operation: None,
            src: None,
        }
    }
}


