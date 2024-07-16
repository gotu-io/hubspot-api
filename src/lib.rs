#![deny(
    unreachable_pub,
    missing_debug_implementations,
    rustdoc::broken_intra_doc_links,
    clippy::all,
    clippy::perf,
    clippy::pedantic,
    clippy::fn_to_numeric_cast_any
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value,
    clippy::match_same_arms,
    clippy::module_name_repetitions,
    clippy::missing_panics_doc,
    clippy::similar_names
)]

use std::sync::Arc;

use builder::HubspotBuilder;
use client::HubspotClient;
use engagements::EngagementsManager;
use objects::ObjectsManager;

mod api_configs;
mod builder;
pub mod client;
mod engagements;
mod objects;
mod owners;

pub mod associations {
    pub use super::api_configs::{AssociationCreationDetails, AssociationTypes};
}

pub use api_configs::types;
pub use engagements::notes;
pub use engagements::EngagementType;
pub use objects::ObjectType;
use owners::OwnerApi;

// A Rust implementation of the Hubspot CRM API
#[derive(Clone, Debug)]
pub struct Hubspot {
    pub portal_id: String,
    /// Objects represent types of relationships or processes.
    pub objects: ObjectsManager,
    /// Engagements store data from interactions with records.
    pub engagements: EngagementsManager,
    /// Owners are specific users assigned to contacts, companies, deals, tickets, or engagements.
    pub owners: OwnerApi,
}

impl Hubspot {
    /// Create hubspot api
    #[must_use]
    pub fn new(client: HubspotClient) -> Self {
        let portal_id = client.portal_id.clone();
        let client = Arc::new(client);

        Self {
            portal_id,
            objects: ObjectsManager::new(Arc::clone(&client)),
            engagements: EngagementsManager::new(Arc::clone(&client)),
            owners: OwnerApi::new(Arc::clone(&client)),
        }
    }

    /// Create Hubspot client
    #[must_use]
    pub fn builder() -> HubspotBuilder {
        HubspotBuilder::new()
    }
}
