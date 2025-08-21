# Retail Chain: Blockchain-Based Supply Chain Management

A decentralized supply chain management system for retail businesses built on the Solana blockchain.

## Overview

Retail Chain is a blockchain solution that creates transparency, trust, and efficiency throughout the retail supply chain process. By leveraging the Solana blockchain's speed and cost-effectiveness, this project enables comprehensive tracking of products from supplier to store, with integration for IoT data, quality verification, and secure supply agreements.

## Features

### Store Management
- Store registration and management
- Product inventory tracking and updates
- Product lifecycle management

### Supplier Management
- Supplier registration and verification
- Rating system for suppliers
- Certification verification
- Product catalog management

### Supply Agreement Management
- Digital agreement creation between stores and suppliers
- Agreement term tracking and enforcement
- Payment tracking
- Dispute resolution mechanisms

### Shipment Tracking
- End-to-end shipment tracking
- Status updates throughout the logistics process
- Delivery verification
- Exception handling and reporting

### IoT Data Integration
- Recording sensor data (temperature, humidity, shock, etc.)
- Data verification and validation
- Real-time condition monitoring

### Event Logging
- Immutable record of all supply chain activities
- Comprehensive audit trail

### Verification System
- Third-party verification capabilities
- Multiple verification levels for enhanced trust

## Technology Stack

- **Blockchain**: Solana
- **Smart Contract Framework**: Anchor
- **Programming Languages**: Rust (on-chain), TypeScript (client/tests)
- **Development Environment**: Anchor CLI

## Getting Started

### Prerequisites
- Rust 1.68+ 
- Solana CLI tools
- Node.js 16+
- Yarn
- Anchor Framework 0.28.0+

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/retail-chain.git
cd retail-chain
```

2. Install dependencies:
```bash
yarn install
```

3. Build the Solana program:
```bash
anchor build
```

### Testing

Run the test suite to verify functionality:
```bash
anchor test
```

### Deployment

Deploy to Solana devnet:
```bash
anchor deploy --provider.cluster devnet
```

## Architecture

The project consists of:

1. **Solana Programs (Smart Contracts)**: Written in Rust, these define the core logic for the supply chain management system.

2. **Account Structures**: 
   - Store
   - Product
   - Supplier
   - SupplyAgreement
   - ShipmentRecord
   - IoTDataRecord
   - VerifierCredential
   - SupplyChainEvent

3. **Status Tracking**:
   - ShipmentStatus (Created, InTransit, Exception, Delivered, Verified)
   - AgreementStatus (Pending, Active, Completed, Disputed, Canceled)
   - EventType (ProductCreated, ShipmentCreated, StatusUpdate, QualityCheck, ComplianceVerification, Payment)
   - IoTDataType (Temperature, Humidity, Location, Shock, LightExposure)

## Use Cases

- **Transparency**: Track products throughout the supply chain
- **Quality Assurance**: Verify products are transported under appropriate conditions
- **Supplier Verification**: Ensure suppliers meet quality and ethical standards
- **Dispute Resolution**: Streamlined process for handling supply issues
- **Regulatory Compliance**: Maintain immutable records for compliance purposes
- **Consumer Trust**: Provide provenance information to end customers

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Solana Foundation
- Anchor Framework team
- All contributors to this project
