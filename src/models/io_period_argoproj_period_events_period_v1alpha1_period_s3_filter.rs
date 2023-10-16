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
pub struct IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodS3Filter {
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

impl IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodS3Filter {
    pub fn new() -> IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodS3Filter {
        IoPeriodArgoprojPeriodEventsPeriodV1alpha1PeriodS3Filter {
            prefix: None,
            suffix: None,
        }
    }
}


