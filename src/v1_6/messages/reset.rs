use crate::v1_6::types::ResetStatus;

/// This contains the field definition of the ResetRequest PDU sent by the Central System to the Charge Point. See also Reset
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResetRequest {
    /// Required. This contains the type of reset that the Charge Point should perform.
    #[serde(rename = "type")]
    pub kind: ResetStatus,
}

/// This contains the field definition of the ResetResponse PDU sent by the Charge Point to the Central System inresponse to a ResetRequest PDU. See also Reset
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResetResponse {
    // Required. This indicates whether the Charge Point is able to perform the reset.
    pub status: ResetStatus,
}