/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodNodeSelectorTerm : A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodNodeSelectorTerm {
    /// A list of node selector requirements by node's labels.
    #[serde(rename = "matchExpressions", skip_serializing_if = "Option::is_none")]
    pub match_expressions: Option<Vec<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodNodeSelectorRequirement>>,
    /// A list of node selector requirements by node's fields.
    #[serde(rename = "matchFields", skip_serializing_if = "Option::is_none")]
    pub match_fields: Option<Vec<crate::models::IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodNodeSelectorRequirement>>,
}

impl IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodNodeSelectorTerm {
    /// A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
    pub fn new() -> IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodNodeSelectorTerm {
        IoPeriodK8sPeriodApiPeriodCorePeriodV1PeriodNodeSelectorTerm {
            match_expressions: None,
            match_fields: None,
        }
    }
}


