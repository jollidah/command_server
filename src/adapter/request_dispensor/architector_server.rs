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
{
    location: 'ams',
    service_type: 'Web',
    computing_service_model: 'IaaS',
    additional_requirements: 'real-time game processing',
    instance_requirements: [
      {
        target_stability: 'Low',
        anticipated_rps: 200,
        requirements_for_data_processing: 'Simple'
      }
    ],
    accessToken: 'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiNWVmYzY1YmYtMDI2OS00ZjZkLWJhZDctZmI3YTNmZDk3N2U2IiwiZW1haWwiOiJqb2xsaWRhaEBnbWFpbC5jb20iLCJleHAiOjE3NDc5NjYxNTIsImlhdCI6MTc0Nzk2NTU1MiwidHlwIjoiQWNjZXNzIn0.lBVXEo0kMUoppbv5Y0XfJyb56m1c8lzoDubJlkLtpKg',
    project_id: 'deb74d75-55ea-484d-a421-c4ae37e684c8'
    }
    Response {
    status: 500,
    statusText: 'Internal Server Error',
    headers: Headers {
      'content-type': 'text/plain; charset=utf-8',
      'content-length': '248',
      vary: 'origin, access-control-request-method, access-control-request-headers',
      'access-control-allow-origin': '*',
      date: 'Fri, 23 May 2025 02:03:14 GMT'
    },
    body: ReadableStream { locked: false, state: 'readable', supportsBYOB: true },
    bodyUsed: false,
    ok: false,
    redirected: false,
    type: 'basic',
    url: 'http://64.176.217.21/command_server/api/v1/external/project/deb74d75-55ea-484d-a421-c4ae37e684c8/architecture/suggestion'
    }