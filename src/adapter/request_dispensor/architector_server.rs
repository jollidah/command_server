use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    config::get_config, domain::project::commands::ResourceResponse, errors::ServiceError,
};

use super::get_client;

pub async fn request_architecture_recommendation(
    user_input: RequestArchitectureSuggestion,
) -> Result<ArchitectureRecommendation, ServiceError> {
    let client = get_client();
    let config = get_config();
    let response = client
        .post(format!(
            "{}/v1/internal/architecture",
            config.architector_server_url
        ))
        .json(&user_input)
        .send()
        .await?;
    Ok(serde_json::from_value::<ArchitectureRecommendation>(
        response.json().await?,
    )?)
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ArchitectureRecommendation {
    rec1: Recommendation,
    rec2: Recommendation,
    rec3: Recommendation,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Recommendation {
    architecture: Vec<ResourceResponse>,
    description: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RequestArchitectureSuggestion {
    location: String,
    service_type: String,
    computing_service_model: String,
    additional_requirements: String,
    instance_requirements: Vec<InstanceRequirement>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct InstanceRequirement {
    target_stability: String,
    anticipated_rps: i32,
    requirements_for_data_processing: String,
}
