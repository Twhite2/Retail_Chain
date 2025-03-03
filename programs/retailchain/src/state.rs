use anchor_lang::prelude::*;

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

// RETAIL ENTITY ACCOUNT STRUCTURES
#[account]
pub struct Store {
    pub owner: Pubkey,
    pub name: String,
    pub location: String,
    pub total_products: u64,
    pub is_active: bool,
}

impl Store {
    pub fn space() -> usize {
        8 +  // discriminator
        32 + // owner: Pubkey
        32 + // name: String (max assumed)
        32 + // location: String (max assumed)
        8 +  // total_products: u64
        1    // is_active: bool
    }
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

impl Product {
    pub fn space() -> usize {
        8 +   // discriminator
        32 +  // store: Pubkey
        32 +  // name: String (max assumed)
        128 + // description: String (max assumed)
        8 +   // price: u64
        8 +   // quantity: u64
        8     // created_at: i64
    }
}

// SUPPLY CHAIN ACCOUNT STRUCTURES
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

impl Supplier {
    pub fn space() -> usize {
        8 +   // discriminator
        32 +  // key: Pubkey
        64 +  // name: String (max assumed)
        128 + // certification: String (max assumed)
        64 +  // description: String (max assumed)
        8 +   // products_supplied: u64
        1 +   // is_verified: bool
        1 +   // rating: u8
        8     // created_at: i64
    }
}

#[account]
pub struct VerifierCredential {
    pub authority: Pubkey,
    pub is_verifier: bool,
    pub verification_level: u8,
    pub organization: String,
}

impl VerifierCredential {
    pub fn space() -> usize {
        8 +   // discriminator
        32 +  // authority: Pubkey
        1 +   // is_verifier: bool
        1 +   // verification_level: u8
        64    // organization: String (max assumed)
    }
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

impl SupplyAgreement {
    pub fn space() -> usize {
        8 +    // discriminator
        32 +   // supplier: Pubkey
        32 +   // store: Pubkey
        256 +  // terms: String (max assumed)
        8 +    // deadline: i64
        8 +    // payment_amount: u64
        1 +    // status: u8
        8 +    // created_at: i64
        64     // products: Vec<Pubkey> (initial allocation for vector)
    }
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

impl ShipmentRecord {
    pub fn space() -> usize {
        8 +    // discriminator
        64 +   // tracking_id: String (max assumed)
        32 +   // supplier: Pubkey
        32 +   // destination: Pubkey
        64 +   // origin_location: String (max assumed)
        64 +   // destination_location: String (max assumed)
        8 +    // created_at: i64
        8 +    // estimated_arrival: i64
        1 +    // status: u8
        256 +  // products: Vec<Pubkey> (sized for 8 products)
        256    // verified_by: Vec<Pubkey> (sized for 8 verifiers)
    }
    
    // Helper method to check if a shipment is verified by a specific authority
    pub fn is_verified_by(&self, authority: &Pubkey) -> bool {
        self.verified_by.contains(authority)
    }
    
    // Helper method to get the current status as enum
    pub fn get_status(&self) -> ShipmentStatus {
        match self.status {
            0 => ShipmentStatus::Created,
            1 => ShipmentStatus::InTransit,
            2 => ShipmentStatus::Exception,
            3 => ShipmentStatus::Delivered,
            4 => ShipmentStatus::Verified,
            _ => ShipmentStatus::Created, // Default fallback
        }
    }
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

impl SupplyChainEvent {
    pub fn space() -> usize {
        8 +    // discriminator
        1 +    // event_type: u8
        32 +   // recorder: Pubkey
        32 +   // related_entity: Pubkey
        64 +   // location: String (max assumed)
        8 +    // timestamp: i64
        256 +  // metadata: String (max assumed)
        8      // created_at: i64
    }
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

impl IoTDataRecord {
    pub fn space() -> usize {
        8 +    // discriminator
        32 +   // shipment: Pubkey
        1 +    // data_type: u8
        128 +  // value: String (max assumed)
        8 +    // timestamp: i64
        32 +   // recorder: Pubkey
        1      // is_verified: bool
    }
    
    // Helper method to get the data type as enum
    pub fn get_data_type(&self) -> IoTDataType {
        match self.data_type {
            0 => IoTDataType::Temperature,
            1 => IoTDataType::Humidity,
            2 => IoTDataType::Location,
            3 => IoTDataType::Shock,
            4 => IoTDataType::LightExposure,
            _ => IoTDataType::Temperature, // Default fallback
        }
    }
}

// Utility functions for state validation
pub fn is_valid_status_transition(current: u8, new: u8) -> bool {
    match (current, new) {
        (s1, s2) if s1 == ShipmentStatus::Created as u8 && s2 == ShipmentStatus::InTransit as u8 => true,
        (s1, s2) if s1 == ShipmentStatus::InTransit as u8 && s2 == ShipmentStatus::Delivered as u8 => true,
        (s1, s2) if s1 == ShipmentStatus::InTransit as u8 && s2 == ShipmentStatus::Exception as u8 => true,
        (s1, s2) if s1 == ShipmentStatus::Exception as u8 && s2 == ShipmentStatus::InTransit as u8 => true,
        (s1, s2) if s1 == ShipmentStatus::Delivered as u8 && s2 == ShipmentStatus::Verified as u8 => true,
        _ => false,
    }
}

// Helper methods for converting between enum and u8
pub trait StatusConversion {
    fn to_u8(&self) -> u8;
    fn from_u8(value: u8) -> Option<Self> where Self: Sized;
}

impl StatusConversion for ShipmentStatus {
    fn to_u8(&self) -> u8 {
        *self as u8
    }
    
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ShipmentStatus::Created),
            1 => Some(ShipmentStatus::InTransit),
            2 => Some(ShipmentStatus::Exception),
            3 => Some(ShipmentStatus::Delivered),
            4 => Some(ShipmentStatus::Verified),
            _ => None,
        }
    }
}

impl StatusConversion for AgreementStatus {
    fn to_u8(&self) -> u8 {
        *self as u8
    }
    
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(AgreementStatus::Pending),
            1 => Some(AgreementStatus::Active),
            2 => Some(AgreementStatus::Completed),
            3 => Some(AgreementStatus::Disputed),
            4 => Some(AgreementStatus::Canceled),
            _ => None,
        }
    }
}
