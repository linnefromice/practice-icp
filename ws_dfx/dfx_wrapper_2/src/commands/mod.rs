use dfx_core::{self, DfxInterface};
use ic_agent::{agent::status::Status, AgentError};

pub async fn ping(interface: &DfxInterface) -> Result<Status, AgentError> {
    interface.agent().status().await
}
