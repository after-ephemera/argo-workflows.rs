/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodHttpHeader : HTTPHeader describes a custom header to be used in HTTP probes



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodHttpHeader {
    /// The header field name
    #[serde(rename = "name")]
    pub name: String,
    /// The header field value
    #[serde(rename = "value")]
    pub value: String,
}

impl IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodHttpHeader {
    /// HTTPHeader describes a custom header to be used in HTTP probes
    pub fn new(name: String, value: String) -> IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodHttpHeader {
        IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodHttpHeader {
            name,
            value,
        }
    }
}


