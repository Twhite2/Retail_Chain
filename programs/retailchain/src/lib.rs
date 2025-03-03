use anchor_lang::prelude::*;

// Import our modules
mod state;
mod errors;
mod instructions;

// Re-export for easier access
pub use state::*;
pub use errors::*;
pub use instructions::*;

declare_id!("7JYPt6XXmADUzAG12ZM3763PuF7XhJmfr7oWV9g2VrcM");

#[program]
pub mod retailchain {
    use super::*;

    // STORE MANAGEMENT
    pub fn initialize_store(
        ctx: Context<InitializeStore>,
        name: String,
        location: String
    ) -> Result<()> {
        instructions::initialize_store(ctx, name, location)
    }

    pub fn add_product(
        ctx: Context<AddProduct>,
        name: String,
        description: String,
        price: u64,
        quantity: u64
    ) -> Result<()> {
        instructions::add_product(ctx, name, description, price, quantity)
    }

    pub fn update_product(
        ctx: Context<UpdateProduct>,
        price: Option<u64>,
        quantity: Option<u64>
    ) -> Result<()> {
        instructions::update_product(ctx, price, quantity)
    }

    // SUPPLIER MANAGEMENT
    pub fn register_supplier(
        ctx: Context<RegisterSupplier>,
        name: String,
        certification: String,
        description: String
    ) -> Result<()> {
        instructions::register_supplier(ctx, name, certification, description)
    }

    pub fn verify_supplier(
        ctx: Context<VerifySupplier>
    ) -> Result<()> {
        instructions::verify_supplier(ctx)
    }

    pub fn update_supplier(
        ctx: Context<UpdateSupplier>,
        certification: Option<String>,
        description: Option<String>
    ) -> Result<()> {
        instructions::update_supplier(ctx, certification, description)
    }

    pub fn rate_supplier(
        ctx: Context<RateSupplier>,
        rating: u8
    ) -> Result<()> {
        instructions::rate_supplier(ctx, rating)
    }

    pub fn add_product_to_supplier_catalog(
        ctx: Context<AddProductToSupplierCatalog>,
        name: String,
        description: String,
        price: u64,
        available_quantity: u64
    ) -> Result<()> {
        instructions::add_product_to_supplier_catalog(ctx, name, description, price, available_quantity)
    }

    // AGREEMENT MANAGEMENT
    pub fn create_supply_agreement(
        ctx: Context<CreateAgreement>,
        terms: String,
        deadline: i64,
        payment_amount: u64
    ) -> Result<()> {
        instructions::create_supply_agreement(ctx, terms, deadline, payment_amount)
    }

    pub fn accept_agreement(
        ctx: Context<UpdateAgreement>
    ) -> Result<()> {
        instructions::accept_agreement(ctx)
    }

    pub fn add_products_to_agreement(
        ctx: Context<AddProductsToAgreement>,
        product_accounts: Vec<Pubkey>
    ) -> Result<()> {
        instructions::add_products_to_agreement(ctx, product_accounts)
    }

    pub fn complete_agreement(
        ctx: Context<CompleteAgreement>
    ) -> Result<()> {
        instructions::complete_agreement(ctx)
    }

    pub fn dispute_agreement(
        ctx: Context<DisputeAgreement>,
        dispute_reason: String
    ) -> Result<()> {
        instructions::dispute_agreement(ctx, dispute_reason)
    }

    pub fn resolve_dispute(
        ctx: Context<ResolveDispute>,
        resolution_notes: String,
        resolution_outcome: u8
    ) -> Result<()> {
        instructions::resolve_dispute(ctx, resolution_notes, resolution_outcome)
    }

    // SHIPMENT MANAGEMENT
    pub fn create_shipment(
        ctx: Context<CreateShipment>,
        tracking_id: String,
        origin_location: String,
        destination_location: String,
        estimated_arrival: i64,
        products: Vec<Pubkey>
    ) -> Result<()> {
        instructions::create_shipment(ctx, tracking_id, origin_location, destination_location, estimated_arrival, products)
    }

    pub fn update_shipment_status(
        ctx: Context<UpdateShipment>,
        new_status: u8
    ) -> Result<()> {
        instructions::update_shipment_status(ctx, new_status)
    }

    pub fn verify_shipment_delivery(
        ctx: Context<VerifyShipmentDelivery>
    ) -> Result<()> {
        instructions::verify_shipment_delivery(ctx)
    }

    pub fn add_shipment_exception(
        ctx: Context<UpdateShipment>,
        exception_details: String
    ) -> Result<()> {
        instructions::add_shipment_exception(ctx, exception_details)
    }

    pub fn record_shipment_location(
        ctx: Context<RecordShipmentLocation>,
        latitude: f64,
        longitude: f64,
        location_name: String
    ) -> Result<()> {
        instructions::record_shipment_location(ctx, latitude, longitude, location_name)
    }

    // IOT DATA MANAGEMENT
    pub fn add_iot_data(
        ctx: Context<AddIoTData>,
        data_type: u8,
        value: String,
        timestamp: i64
    ) -> Result<()> {
        instructions::add_iot_data(ctx, data_type, value, timestamp)
    }

    pub fn verify_iot_data(
        ctx: Context<VerifyIoTData>
    ) -> Result<()> {
        instructions::verify_iot_data(ctx)
    }

    // SUPPLY CHAIN EVENT LOGGING
    pub fn record_supply_chain_event(
        ctx: Context<RecordEvent>,
        event_type: u8,
        location: String,
        timestamp: i64,
        metadata: String
    ) -> Result<()> {
        instructions::record_supply_chain_event(ctx, event_type, location, timestamp, metadata)
    }

    // VERIFIER MANAGEMENT
    pub fn register_verifier(
        ctx: Context<RegisterVerifier>,
        verification_level: u8,
        organization: String
    ) -> Result<()> {
        instructions::register_verifier(ctx, verification_level, organization)
    }
}