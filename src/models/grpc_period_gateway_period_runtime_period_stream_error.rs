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
pub struct GrpcPeriodGatewayPeriodRuntimePeriodStreamError {
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<crate::models::GooglePeriodProtobufPeriodAny>>,
    #[serde(rename = "grpc_code", skip_serializing_if = "Option::is_none")]
    pub grpc_code: Option<i32>,
    #[serde(rename = "http_code", skip_serializing_if = "Option::is_none")]
    pub http_code: Option<i32>,
    #[serde(rename = "http_status", skip_serializing_if = "Option::is_none")]
    pub http_status: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl GrpcPeriodGatewayPeriodRuntimePeriodStreamError {
    pub fn new() -> GrpcPeriodGatewayPeriodRuntimePeriodStreamError {
        GrpcPeriodGatewayPeriodRuntimePeriodStreamError {
            details: None,
            grpc_code: None,
            http_code: None,
            http_status: None,
            message: None,
        }
    }
}


