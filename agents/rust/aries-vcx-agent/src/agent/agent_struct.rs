use std::sync::Arc;

use aries_vcx::agency_client::agency_client::AgencyClient;
use aries_vcx::vdrtools_sys::{PoolHandle, WalletHandle};

use crate::agent::agent_config::AgentConfig;

use crate::error::*;
use crate::services::{
    connection::ServiceConnections, credential_definition::ServiceCredentialDefinitions,
    holder::ServiceCredentialsHolder, issuer::ServiceCredentialsIssuer, prover::ServiceProver,
    revocation_registry::ServiceRevocationRegistries, schema::ServiceSchemas,
    verifier::ServiceVerifier,
};

#[derive(Clone)]
pub struct Agent {
    pub(super) wallet_handle: WalletHandle,
    pub(super) pool_handle: PoolHandle,
    pub(super) config: AgentConfig,
    pub(super) connections: Arc<ServiceConnections>,
    pub(super) schemas: Arc<ServiceSchemas>,
    pub(super) cred_defs: Arc<ServiceCredentialDefinitions>,
    pub(super) rev_regs: Arc<ServiceRevocationRegistries>,
    pub(super) holder: Arc<ServiceCredentialsHolder>,
    pub(super) issuer: Arc<ServiceCredentialsIssuer>,
    pub(super) verifier: Arc<ServiceVerifier>,
    pub(super) prover: Arc<ServiceProver>,
}

impl Agent {
    pub fn wallet_handle(&self) -> WalletHandle {
        self.wallet_handle
    }

    pub fn pool_handle(&self) -> PoolHandle {
        self.pool_handle
    }

    pub fn agent_config(&self) -> AgentConfig {
        self.config.clone()
    }

    pub fn issuer_did(&self) -> String {
        self.config.config_issuer.institution_did.clone()
    }

    pub fn agency_client(&self) -> AgentResult<AgencyClient> {
        AgencyClient::new()
            .configure(self.wallet_handle, &self.config.config_agency_client)
            .map_err(|err| {
                AgentError::from_msg(
                    AgentErrorKind::GenericAriesVcxError,
                    &format!("Failed to configure agency client: {}", err),
                )
            })
    }

    pub fn connections(&self) -> Arc<ServiceConnections> {
        self.connections.clone()
    }

    pub fn schemas(&self) -> Arc<ServiceSchemas> {
        self.schemas.clone()
    }

    pub fn cred_defs(&self) -> Arc<ServiceCredentialDefinitions> {
        self.cred_defs.clone()
    }

    pub fn rev_regs(&self) -> Arc<ServiceRevocationRegistries> {
        self.rev_regs.clone()
    }

    pub fn issuer(&self) -> Arc<ServiceCredentialsIssuer> {
        self.issuer.clone()
    }

    pub fn holder(&self) -> Arc<ServiceCredentialsHolder> {
        self.holder.clone()
    }

    pub fn verifier(&self) -> Arc<ServiceVerifier> {
        self.verifier.clone()
    }

    pub fn prover(&self) -> Arc<ServiceProver> {
        self.prover.clone()
    }
}
