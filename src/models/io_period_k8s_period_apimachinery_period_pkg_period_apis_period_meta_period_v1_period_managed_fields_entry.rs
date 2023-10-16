/*
 * Argo Workflows API
 *
 * Argo Workflows is an open source container-native workflow engine for orchestrating parallel jobs on Kubernetes. For more information, please see https://argoproj.github.io/argo-workflows/
 *
 * The version of the OpenAPI document: VERSION
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoPeriodK8sPeriodApimachineryPeriodPkgPeriodApisPeriodMetaPeriodV1PeriodManagedFieldsEntry : ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource that the fieldset applies to.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoPeriodK8sPeriodApimachineryPeriodPkgPeriodApisPeriodMetaPeriodV1PeriodManagedFieldsEntry {
    /// APIVersion defines the version of this resource that this field set applies to. The format is \"group/version\" just like the top-level APIVersion field. It is necessary to track the version of a field set because it cannot be automatically converted.
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// FieldsType is the discriminator for the different fields format and version. There is currently only one possible value: \"FieldsV1\"
    #[serde(rename = "fieldsType", skip_serializing_if = "Option::is_none")]
    pub fields_type: Option<String>,
    /// FieldsV1 stores a set of fields in a data structure like a Trie, in JSON format.  Each key is either a '.' representing the field itself, and will always map to an empty set, or a string representing a sub-field or item. The string will follow one of these four formats: 'f:<name>', where <name> is the name of a field in a struct, or key in a map 'v:<value>', where <value> is the exact json formatted value of a list item 'i:<index>', where <index> is position of a item in a list 'k:<keys>', where <keys> is a map of  a list item's key fields to their unique values If a key maps to an empty Fields value, the field that key represents is part of the set.  The exact format is defined in sigs.k8s.io/structured-merge-diff
    #[serde(rename = "fieldsV1", skip_serializing_if = "Option::is_none")]
    pub fields_v1: Option<serde_json::Value>,
    /// Manager is an identifier of the workflow managing these fields.
    #[serde(rename = "manager", skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    /// Operation is the type of operation which lead to this ManagedFieldsEntry being created. The only valid values for this field are 'Apply' and 'Update'.
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// Subresource is the name of the subresource used to update that object, or empty string if the object was updated through the main resource. The value of this field is used to distinguish between managers, even if they share the same name. For example, a status update will be distinct from a regular update using the same manager name. Note that the APIVersion field is not related to the Subresource field and it always corresponds to the version of the main resource.
    #[serde(rename = "subresource", skip_serializing_if = "Option::is_none")]
    pub subresource: Option<String>,
    /// Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

impl IoPeriodK8sPeriodApimachineryPeriodPkgPeriodApisPeriodMetaPeriodV1PeriodManagedFieldsEntry {
    /// ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource that the fieldset applies to.
    pub fn new() -> IoPeriodK8sPeriodApimachineryPeriodPkgPeriodApisPeriodMetaPeriodV1PeriodManagedFieldsEntry {
        IoPeriodK8sPeriodApimachineryPeriodPkgPeriodApisPeriodMetaPeriodV1PeriodManagedFieldsEntry {
            api_version: None,
            fields_type: None,
            fields_v1: None,
            manager: None,
            operation: None,
            subresource: None,
            time: None,
        }
    }
}


