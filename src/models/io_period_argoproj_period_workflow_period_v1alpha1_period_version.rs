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
pub struct IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodVersion {
    #[serde(rename = "buildDate")]
    pub build_date: String,
    #[serde(rename = "compiler")]
    pub compiler: String,
    #[serde(rename = "gitCommit")]
    pub git_commit: String,
    #[serde(rename = "gitTag")]
    pub git_tag: String,
    #[serde(rename = "gitTreeState")]
    pub git_tree_state: String,
    #[serde(rename = "goVersion")]
    pub go_version: String,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "version")]
    pub version: String,
}

impl IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodVersion {
    pub fn new(build_date: String, compiler: String, git_commit: String, git_tag: String, git_tree_state: String, go_version: String, platform: String, version: String) -> IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodVersion {
        IoPeriodArgoprojPeriodWorkflowPeriodV1alpha1PeriodVersion {
            build_date,
            compiler,
            git_commit,
            git_tag,
            git_tree_state,
            go_version,
            platform,
            version,
        }
    }
}


