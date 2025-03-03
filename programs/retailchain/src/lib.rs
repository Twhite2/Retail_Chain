use anchor_lang::prelude::*;

declare_id!("7JYPt6XXmADUzAG12ZM3763PuF7XhJmfr7oWV9g2VrcM");

#[program]
pub mod retailchain {
    use super::*;

    pub fn initialize_store(
        ctx: Context<InitializeStore>,
        name: String,
        location: String
    ) -> Result<()> {
        let store = &mut ctx.accounts.store;
        store.owner = ctx.accounts.owner.key();
        store.name = name;
        store.location = location;
        store.total_products = 0;
        store.is_active = true;
        Ok(())
    }

    pub fn add_product(
        ctx: Context<AddProduct>,
        name: String,
        description: String,
        price: u64,
        quantity: u64
    ) -> Result<()> {
        let product = &mut ctx.accounts.product;
        let store = &mut ctx.accounts.store;

        product.store = store.key();
        product.name = name;
        product.description = description;
        product.price = price;
        product.quantity = quantity;
        product.created_at = Clock::get()?.unix_timestamp;
        
        store.total_products = store.total_products.checked_add(1).unwrap();
        
        Ok(())
    }

    pub fn update_product(
        ctx: Context<UpdateProduct>,
        price: Option<u64>,
        quantity: Option<u64>
    ) -> Result<()> {
        let product = &mut ctx.accounts.product;
        
        if let Some(new_price) = price {
            product.price = new_price;
        }
        
        if let Some(new_quantity) = quantity {
            product.quantity = new_quantity;
        }
        
        Ok(())
    }

    pub fn register_supplier(
        ctx: Context<RegisterSupplier>,
        name: String,
        certification: String,
        description: String
    ) -> Result<()> {
        let supplier = &mut ctx.accounts.supplier;
        supplier.key = ctx.accounts.authority.key();
        supplier.name = name;
        supplier.certification = certification;
        supplier.description = description;
        supplier.products_supplied = 0;
        supplier.is_verified = false;
        supplier.rating = 0;
        supplier.created_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn verify_supplier(
        ctx: Context<VerifySupplier>
    ) -> Result<()> {
        let supplier = &mut ctx.accounts.supplier;
        
        // Only allow verification by authorized verifiers
        require!(
            ctx.accounts.authority.key() == supplier.key || 
            ctx.accounts.authority_credentials.is_verifier, 
            SupplyChainError::UnauthorizedVerifier
        );
        
        supplier.is_verified = true;
        
        Ok(())
    }

    pub fn create_supply_agreement(
        ctx: Context<CreateAgreement>,
        terms: String,
        deadline: i64,
        payment_amount: u64
    ) -> Result<()> {
        let agreement = &mut ctx.accounts.agreement;
        
        agreement.supplier = ctx.accounts.supplier.key();
        agreement.store = ctx.accounts.store.key();
        agreement.terms = terms;
        agreement.deadline = deadline;
        agreement.payment_amount = payment_amount;
        agreement.status = AgreementStatus::Pending as u8;
        agreement.created_at = Clock::get()?.unix_timestamp;
        agreement.products = Vec::new();
        
        Ok(())
    }

    pub fn accept_agreement(
        ctx: Context<UpdateAgreement>
    ) -> Result<()> {
        let agreement = &mut ctx.accounts.agreement;
        
        require!(
            agreement.status == AgreementStatus::Pending as u8,
            SupplyChainError::InvalidAgreementStatus
        );
        
        agreement.status = AgreementStatus::Active as u8;
        
        Ok(())
    }

    pub fn create_shipment(
        ctx: Context<CreateShipment>,
        tracking_id: String,
        origin_location: String,
        destination_location: String,
        estimated_arrival: i64,
        products: Vec<Pubkey>
    ) -> Result<()> {
        let shipment = &mut ctx.accounts.shipment;
        
        shipment.tracking_id = tracking_id;
        shipment.supplier = ctx.accounts.supplier.key();
        shipment.destination = ctx.accounts.store.key();
        shipment.origin_location = origin_location;
        shipment.destination_location = destination_location;
        shipment.created_at = Clock::get()?.unix_timestamp;
        shipment.estimated_arrival = estimated_arrival;
        shipment.status = ShipmentStatus::Created as u8;
        shipment.products = products;
        shipment.verified_by = Vec::new();
        
        Ok(())
    }

    pub fn update_shipment_status(
        ctx: Context<UpdateShipment>,
        new_status: u8
    ) -> Result<()> {
        let shipment = &mut ctx.accounts.shipment;
        
        // Validate status transition
        require!(
            is_valid_status_transition(shipment.status, new_status),
            SupplyChainError::InvalidStatusTransition
        );
        
        shipment.status = new_status;
        
        // Record the authority that updated the status
        if !shipment.verified_by.contains(&ctx.accounts.authority.key()) {
            shipment.verified_by.push(ctx.accounts.authority.key());
        }
        
        Ok(())
    }

    pub fn record_supply_chain_event(
        ctx: Context<RecordEvent>,
        event_type: u8,
        location: String,
        timestamp: i64,
        metadata: String
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;
        
        event.event_type = event_type;
        event.recorder = ctx.accounts.authority.key();
        event.related_entity = ctx.accounts.related_entity.key();
        event.location = location;
        event.timestamp = timestamp;
        event.metadata = metadata;
        event.created_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn add_iot_data(
        ctx: Context<AddIoTData>,
        data_type: u8,
        value: String,
        timestamp: i64
    ) -> Result<()> {
        let iot_record = &mut ctx.accounts.iot_record;
        
        iot_record.shipment = ctx.accounts.shipment.key();
        iot_record.data_type = data_type;
        iot_record.value = value;
        iot_record.timestamp = timestamp;
        iot_record.recorder = ctx.accounts.authority.key();
        iot_record.is_verified = false;
        
        Ok(())
    }

    pub fn verify_iot_data(
        ctx: Context<VerifyIoTData>
    ) -> Result<()> {
        let iot_record = &mut ctx.accounts.iot_record;
        
        require!(
            ctx.accounts.authority_credentials.is_verifier,
            SupplyChainError::UnauthorizedVerifier
        );
        
        iot_record.is_verified = true;
        
        Ok(())
    }
}

// Helper function for state transitions
fn is_valid_status_transition(current: u8, new: u8) -> bool {
    match (current, new) {
        (s1, s2) if s1 == ShipmentStatus::Created as u8 && s2 == ShipmentStatus::InTransit as u8 => true,
        (s1, s2) if s1 == ShipmentStatus::InTransit as u8 && s2 == ShipmentStatus::Delivered as u8 => true,
        (s1, s2) if s1 == ShipmentStatus::InTransit as u8 && s2 == ShipmentStatus::Exception as u8 => true,
        (s1, s2) if s1 == ShipmentStatus::Exception as u8 && s2 == ShipmentStatus::InTransit as u8 => true,
        (s1, s2) if s1 == ShipmentStatus::Delivered as u8 && s2 == ShipmentStatus::Verified as u8 => true,
        _ => false,
    }
}

#[error_code]
pub enum SupplyChainError {
    #[msg("Operation not permitted for the current status")]
    InvalidStatusTransition,
    #[msg("Only authorized verifiers can perform this action")]
    UnauthorizedVerifier,
    #[msg("Agreement is not in the correct status for this operation")]
    InvalidAgreementStatus,
    #[msg("Data provided is outside acceptable parameters")]
    InvalidData,
    #[msg("Required verification has not been completed")]
    VerificationRequired,
}

// ENUM DEFINITIONS
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ShipmentStatus {
    Created = 0,
    InTransit = 1,
    Exception = 2,
    Delivered = 3,
    Verified = 4,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AgreementStatus {
    Pending = 0,
    Active = 1,
    Completed = 2,
    Disputed = 3,
    Canceled = 4,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EventType {
    ProductCreated = 0,
    ShipmentCreated = 1,
    StatusUpdate = 2,
    QualityCheck = 3,
    ComplianceVerification = 4,
    Payment = 5,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum IoTDataType {
    Temperature = 0,
    Humidity = 1,
    Location = 2,
    Shock = 3,
    LightExposure = 4,
}

// EXISTING ACCOUNT STRUCTURES
#[account]
pub struct Store {
    pub owner: Pubkey,
    pub name: String,
    pub location: String,
    pub total_products: u64,
    pub is_active: bool,
}

#[account]
pub struct Product {
    pub store: Pubkey,
    pub name: String,
    pub description: String,
    pub price: u64,
    pub quantity: u64,
    pub created_at: i64,
}

// EXPANDED ACCOUNT STRUCTURES
#[account]
pub struct Supplier {
    pub key: Pubkey,
    pub name: String,
    pub certification: String,
    pub description: String,
    pub products_supplied: u64,
    pub is_verified: bool,
    pub rating: u8,
    pub created_at: i64,
}

#[account]
pub struct VerifierCredential {
    pub authority: Pubkey,
    pub is_verifier: bool,
    pub verification_level: u8,
    pub organization: String,
}

#[account]
pub struct SupplyAgreement {
    pub supplier: Pubkey,
    pub store: Pubkey,
    pub terms: String,
    pub deadline: i64,
    pub payment_amount: u64,
    pub status: u8,
    pub created_at: i64,
    pub products: Vec<Pubkey>,
}

#[account]
pub struct ShipmentRecord {
    pub tracking_id: String,
    pub supplier: Pubkey,
    pub destination: Pubkey,
    pub origin_location: String,
    pub destination_location: String,
    pub created_at: i64,
    pub estimated_arrival: i64,
    pub status: u8,
    pub products: Vec<Pubkey>,
    pub verified_by: Vec<Pubkey>,
}

#[account]
pub struct SupplyChainEvent {
    pub event_type: u8,
    pub recorder: Pubkey,
    pub related_entity: Pubkey,
    pub location: String,
    pub timestamp: i64,
    pub metadata: String,
    pub created_at: i64,
}

#[account]
pub struct IoTDataRecord {
    pub shipment: Pubkey,
    pub data_type: u8,
    pub value: String,
    pub timestamp: i64,
    pub recorder: Pubkey,
    pub is_verified: bool,
}

// EXISTING CONTEXT STRUCTURES
#[derive(Accounts)]
pub struct InitializeStore<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + 32 + 8 + 1
    )]
    pub store: Account<'info, Store>,
    pub system_program: Program<'info, System>,
}

// NEW CONTEXT STRUCTURES
#[derive(Accounts)]
pub struct RegisterSupplier<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 64 + 128 + 64 + 8 + 1 + 1 + 8
    )]
    pub supplier: Account<'info, Supplier>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifySupplier<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = authority_credentials.authority == authority.key(),
        constraint = authority_credentials.is_verifier
    )]
    pub authority_credentials: Account<'info, VerifierCredential>,
    #[account(mut)]
    pub supplier: Account<'info, Supplier>,
}

#[derive(Accounts)]
pub struct CreateAgreement<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = supplier.key == authority.key() || store.owner == authority.key()
    )]
    pub supplier: Account<'info, Supplier>,
    pub store: Account<'info, Store>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 256 + 8 + 8 + 1 + 8 + 64
    )]
    pub agreement: Account<'info, SupplyAgreement>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAgreement<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = (agreement.supplier == authority.key() && supplier.key == authority.key()) || 
                    (agreement.store == store.key() && store.owner == authority.key())
    )]
    pub agreement: Account<'info, SupplyAgreement>,
    pub supplier: Account<'info, Supplier>,
    pub store: Account<'info, Store>,
}

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
        space = 8 + 64 + 32 + 32 + 64 + 64 + 8 + 8 + 1 + 256 + 256
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
        constraint = (shipment.supplier == supplier.key && supplier.key == authority.key()) || 
                    (shipment.destination == store.key() && store.owner == authority.key()) ||
                    authority_credentials.is_verifier
    )]
    pub shipment: Account<'info, ShipmentRecord>,
    pub supplier: Account<'info, Supplier>,
    pub store: Account<'info, Store>,
    pub authority_credentials: Option<Account<'info, VerifierCredential>>,
}

#[derive(Accounts)]
pub struct RecordEvent<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub related_entity: AccountInfo<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 1 + 32 + 32 + 64 + 8 + 256 + 8
    )]
    pub event: Account<'info, SupplyChainEvent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddIoTData<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub shipment: Account<'info, ShipmentRecord>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1 + 128 + 8 + 32 + 1
    )]
    pub iot_record: Account<'info, IoTDataRecord>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyIoTData<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = authority_credentials.authority == authority.key(),
        constraint = authority_credentials.is_verifier
    )]
    pub authority_credentials: Account<'info, VerifierCredential>,
    #[account(mut)]
    pub iot_record: Account<'info, IoTDataRecord>,
}