use ic_agent::{agent::status::Status, Agent, AgentError};

pub async fn ping(agent: &Agent) -> Result<Status, AgentError> {
    agent.status().await
}
