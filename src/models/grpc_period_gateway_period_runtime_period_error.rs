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
pub struct GrpcPeriodGatewayPeriodRuntimePeriodError {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<crate::models::GooglePeriodProtobufPeriodAny>>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl GrpcPeriodGatewayPeriodRuntimePeriodError {
    pub fn new() -> GrpcPeriodGatewayPeriodRuntimePeriodError {
        GrpcPeriodGatewayPeriodRuntimePeriodError {
            code: None,
            details: None,
            error: None,
            message: None,
        }
    }
}


