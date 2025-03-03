use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::SupplyChainError;
use super::{validate_supplier_verified, validate_verifier_credentials};

// Shipment creation and management
pub fn create_shipment(
    ctx: Context<CreateShipment>,
    tracking_id: String,
    origin_location: String,
    destination_location: String,
    estimated_arrival: i64,
    products: Vec<Pubkey>
) -> Result<()> {
    let shipment = &mut ctx.accounts.shipment;
    
    // Validate tracking ID format (example validation)
    require!(
        !tracking_id.is_empty() && tracking_id.len() <= 32,
        SupplyChainError::InvalidData
    );
    
    // Validate estimated arrival time is in the future
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        estimated_arrival > current_time,
        SupplyChainError::InvalidData
    );
    
    // Initialize the shipment record
    shipment.tracking_id = tracking_id;
    shipment.supplier = ctx.accounts.supplier.key();
    shipment.destination = ctx.accounts.store.key();
    shipment.origin_location = origin_location;
    shipment.destination_location = destination_location;
    shipment.created_at = current_time;
    shipment.estimated_arrival = estimated_arrival;
    shipment.status = ShipmentStatus::Created as u8;
    shipment.products = products;
    shipment.verified_by = Vec::new();
    
    // Record the creation event
    emit!(ShipmentCreatedEvent {
        shipment: shipment.key(),
        supplier: shipment.supplier,
        destination: shipment.destination,
        tracking_id: shipment.tracking_id.clone(),
        estimated_arrival,
        timestamp: current_time,
    });
    
    Ok(())
}

pub fn update_shipment_status(
    ctx: Context<UpdateShipment>,
    new_status: u8
) -> Result<()> {
    let shipment = &mut ctx.accounts.shipment;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate status transition
    require!(
        is_valid_status_transition(shipment.status, new_status),
        SupplyChainError::InvalidStatusTransition
    );
    
    // Update status
    let old_status = shipment.status;
    shipment.status = new_status;
    
    // Record the authority that updated the status
    if !shipment.verified_by.contains(&ctx.accounts.authority.key()) {
        shipment.verified_by.push(ctx.accounts.authority.key());
    }
    
    // If transitioning to Delivered status, verify products received
    if new_status == ShipmentStatus::Delivered as u8 {
        // Any additional verification logic here
    }
    
    // Record the status update event
    emit!(ShipmentStatusUpdatedEvent {
        shipment: shipment.key(),
        old_status,
        new_status,
        updated_by: ctx.accounts.authority.key(),
        timestamp: current_time,
    });
    
    Ok(())
}

pub fn verify_shipment_delivery(
    ctx: Context<VerifyShipmentDelivery>
) -> Result<()> {
    let shipment = &mut ctx.accounts.shipment;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Ensure shipment is in Delivered status before verification
    require!(
        shipment.status == ShipmentStatus::Delivered as u8,
        SupplyChainError::InvalidStatusTransition
    );
    
    // Update to Verified status
    shipment.status = ShipmentStatus::Verified as u8;
    
    // Add verifier to the list if not already present
    if !shipment.verified_by.contains(&ctx.accounts.authority.key()) {
        shipment.verified_by.push(ctx.accounts.authority.key());
    }
    
    // Record verification event
    emit!(ShipmentVerifiedEvent {
        shipment: shipment.key(),
        verified_by: ctx.accounts.authority.key(),
        timestamp: current_time,
    });
    
    // If this verification completes an agreement, update the agreement status
    if let Some(agreement) = &ctx.accounts.agreement {
        if agreement.status == AgreementStatus::Active as u8 {
            // Logic to check if all shipments for this agreement are verified
            // For simplicity, we're assuming one shipment per agreement here
            ctx.accounts.agreement.as_mut().unwrap().status = AgreementStatus::Completed as u8;
            
            emit!(AgreementCompletedEvent {
                agreement: agreement.key(),
                completed_at: current_time,
            });
        }
    }
    
    Ok(())
}

pub fn add_shipment_exception(
    ctx: Context<UpdateShipment>,
    exception_details: String
) -> Result<()> {
    let shipment = &mut ctx.accounts.shipment;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Can only add exceptions to shipments in transit or already in exception state
    require!(
        shipment.status == ShipmentStatus::InTransit as u8 || 
        shipment.status == ShipmentStatus::Exception as u8,
        SupplyChainError::InvalidStatusTransition
    );
    
    // Set shipment to exception status
    shipment.status = ShipmentStatus::Exception as u8;
    
    // Record the authority that reported the exception
    if !shipment.verified_by.contains(&ctx.accounts.authority.key()) {
        shipment.verified_by.push(ctx.accounts.authority.key());
    }
    
    // Record exception event
    emit!(ShipmentExceptionEvent {
        shipment: shipment.key(),
        reported_by: ctx.accounts.authority.key(),
        details: exception_details,
        timestamp: current_time,
    });
    
    Ok(())
}

// Account contexts for shipment operations
#[derive(Accounts)]
pub struct CreateShipment<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = supplier.key == authority.key(),
        constraint = supplier.is_verified
    )]
    pub supplier: Account<'info, Supplier>,
    pub store: Account<'info, Store>,
    #[account(
        init,
        payer = authority,
        space = ShipmentRecord::space()
    )]
    pub shipment: Account<'info, ShipmentRecord>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateShipment<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = (shipment.supplier == supplier.key() && supplier.key() == authority.key()) || 
                    (shipment.destination == store.key() && store.owner == authority.key()) ||
                    (authority_credentials.is_some() && 
                     authority_credentials.as_ref().unwrap().authority == authority.key() &&
                     authority_credentials.as_ref().unwrap().is_verifier)
    )]
    pub shipment: Account<'info, ShipmentRecord>,
    pub supplier: Account<'info, Supplier>,
    pub store: Account<'info, Store>,
    pub authority_credentials: Option<Account<'info, VerifierCredential>>,
}

#[derive(Accounts)]
pub struct VerifyShipmentDelivery<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = shipment.destination == store.key() && store.owner == authority.key(),
        constraint = shipment.status == ShipmentStatus::Delivered as u8
    )]
    pub shipment: Account<'info, ShipmentRecord>,
    pub store: Account<'info, Store>,
    pub supplier: Account<'info, Supplier>,
    pub agreement: Option<Account<'info, SupplyAgreement>>,
}

// Event definitions
#[event]
pub struct ShipmentCreatedEvent {
    pub shipment: Pubkey,
    pub supplier: Pubkey,
    pub destination: Pubkey,
    pub tracking_id: String,
    pub estimated_arrival: i64,
    pub timestamp: i64,
}

#[event]
pub struct ShipmentStatusUpdatedEvent {
    pub shipment: Pubkey,
    pub old_status: u8,
    pub new_status: u8,
    pub updated_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ShipmentVerifiedEvent {
    pub shipment: Pubkey,
    pub verified_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ShipmentExceptionEvent {
    pub shipment: Pubkey,
    pub reported_by: Pubkey,
    pub details: String,
    pub timestamp: i64,
}

#[event]
pub struct AgreementCompletedEvent {
    pub agreement: Pubkey,
    pub completed_at: i64,
}

// Shipment tracking functionality
pub fn record_shipment_location(
    ctx: Context<RecordShipmentLocation>,
    latitude: f64,
    longitude: f64,
    location_name: String
) -> Result<()> {
    let location_record = &mut ctx.accounts.location_record;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate the shipment is in a status that allows location updates
    require!(
        ctx.accounts.shipment.status == ShipmentStatus::InTransit as u8 || 
        ctx.accounts.shipment.status == ShipmentStatus::Exception as u8,
        SupplyChainError::InvalidStatusTransition
    );
    
    // Record location data
    location_record.shipment = ctx.accounts.shipment.key();
    location_record.latitude = latitude;
    location_record.longitude = longitude;
    location_record.location_name = location_name;
    location_record.timestamp = current_time;
    location_record.recorded_by = ctx.accounts.authority.key();
    
    // Emit event for tracking
    emit!(ShipmentLocationEvent {
        shipment: ctx.accounts.shipment.key(),
        latitude,
        longitude,
        location_name: location_name.clone(),
        timestamp: current_time,
    });
    
    Ok(())
}

#[derive(Accounts)]
pub struct RecordShipmentLocation<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = (shipment.supplier == authority.key()) || 
                    (authority_credentials.is_some() && 
                     authority_credentials.as_ref().unwrap().authority == authority.key() &&
                     authority_credentials.as_ref().unwrap().is_verifier)
    )]
    pub shipment: Account<'info, ShipmentRecord>,
    #[account(
        init,
        payer = authority,
        space = ShipmentLocation::space()
    )]
    pub location_record: Account<'info, ShipmentLocation>,
    pub authority_credentials: Option<Account<'info, VerifierCredential>>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct ShipmentLocationEvent {
    pub shipment: Pubkey,
    pub latitude: f64,
    pub longitude: f64,
    pub location_name: String,
    pub timestamp: i64,
}

// Additional account structure for location tracking
#[account]
pub struct ShipmentLocation {
    pub shipment: Pubkey,
    pub latitude: f64,
    pub longitude: f64,
    pub location_name: String,
    pub timestamp: i64,
    pub recorded_by: Pubkey,
}

impl ShipmentLocation {
    pub fn space() -> usize {
        8 +    // discriminator
        32 +   // shipment: Pubkey
        8 +    // latitude: f64
        8 +    // longitude: f64
        64 +   // location_name: String (max assumed)
        8 +    // timestamp: i64
        32     // recorded_by: Pubkey
    }
}
