pub mod supplier;
pub mod shipment;
pub mod agreement;
pub mod store;
pub mod product;
pub mod iot;
pub mod events;

// Re-export instruction handlers for cleaner imports in lib.rs
pub use supplier::*;
pub use shipment::*;
pub use agreement::*;
pub use store::*;
pub use product::*;
pub use iot::*;
pub use events::*;

// Common instruction context utilities
use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::SupplyChainError;

// Shared validation functions that may be used across different instruction modules

/// Validates that a user has permission to modify a store
pub fn validate_store_authority(
    authority: &Signer,
    store: &Account<Store>,
) -> Result<()> {
    require!(
        store.owner == authority.key(),
        SupplyChainError::Unauthorized
    );
    Ok(())
}

/// Validates that a store is active
pub fn validate_store_active(store: &Account<Store>) -> Result<()> {
    require!(store.is_active, SupplyChainError::StoreInactive);
    Ok(())
}

/// Validates that a supplier is verified
pub fn validate_supplier_verified(supplier: &Account<Supplier>) -> Result<()> {
    require!(
        supplier.is_verified,
        SupplyChainError::VerificationRequired
    );
    Ok(())
}

/// Validates entity relationships in the supply chain
pub fn validate_supply_chain_relationship(
    supplier: &Account<Supplier>,
    store: &Account<Store>,
    agreement: &Account<SupplyAgreement>,
) -> Result<()> {
    require!(
        agreement.supplier == supplier.key() && 
        agreement.store == store.key() &&
        agreement.status == AgreementStatus::Active as u8,
        SupplyChainError::InvalidRelationship
    );
    Ok(())
}

/// Validates that a verifier has appropriate credentials
pub fn validate_verifier_credentials(
    authority: &Signer,
    credentials: &Account<VerifierCredential>,
) -> Result<()> {
    require!(
        credentials.authority == authority.key() && 
        credentials.is_verifier,
        SupplyChainError::UnauthorizedVerifier
    );
    Ok(())
}
