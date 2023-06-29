use serde::{Deserialize, Serialize};
use std::{
    env,
    error::Error,
    fmt::Display,
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub enum PaymentStatus {
    Paid,
    #[default]
    Unpaid,
}
impl Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentStatus::Paid => write!(f, "PAID"),
            PaymentStatus::Unpaid => write!(f, "UNPAID"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
#[serde(rename_all = "camelCase")]
pub struct FontSizes {
    pub small: f64,
    pub medium: f64,
    pub large: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceStructure {
    pub font_sizes: FontSizes,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Partner {
    pub id: i32,
    pub partner_name: String,
    pub partner_address: String,
    pub partner_postal_code: String,
    pub partner_vat_id: String,
    pub is_vat_payer: bool,
}
impl Partner {
    pub fn default() -> Self {
        Self {
            id: rand::random::<i32>(),
            partner_name: "Partner".to_string(),
            partner_address: "Address".to_string(),
            partner_postal_code: "0000".to_string(),
            partner_vat_id: "00000000".to_string(),
            is_vat_payer: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: i32,
    pub service_name: String,
    pub service_quantity: i32,
    pub service_price: f64,
}
impl Service {
    pub fn default() -> Self {
        Self {
            id: rand::random::<i32>(),
            service_name: "".to_string(),
            service_quantity: 1,
            service_price: 0.0,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub id: i32,
    pub company_currency: String,
    pub company_name: String,
    pub company_address: String,
    pub company_postal_code: String,
    pub company_bankname: String,
    pub company_vat_id: String,
    pub company_iban: String,
    pub company_swift: String,
    pub company_registration_number: String,
    pub company_phone: String,
    pub company_signature: Option<String>,
    pub company_signature_path: Option<PathBuf>, //Base64 string
    pub company_vat_rate: f64,
    pub company_business_registered_at: String,
}
impl Company {
    pub fn default() -> Self {
        Self {
            id: rand::random::<i32>(),
            company_currency: "EUR".to_string(),
            company_name: "Company".to_string(),
            company_address: "Address".to_string(),
            company_postal_code: "0000".to_string(),
            company_bankname: "Bank".to_string(),
            company_vat_id: "00000000".to_string(),
            company_iban: "00000000000000000000".to_string(),
            company_swift: "000000000".to_string(),
            company_registration_number: "00000000000000000000".to_string(),
            company_phone: "000000000".to_string(),
            company_signature_path: None,
            company_signature: None,
            company_vat_rate: 0.0,
            company_business_registered_at: "000000000".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Racun {
    pub invoice: Invoice,
    pub config: InvoiceStructure,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub id: i32,
    pub invoice_number: String,
    pub invoice_date: String,
    pub invoice_location: String,
    pub service_date: String,
    pub invoice_currency: String,
    pub due_date: String,
    pub partner: Partner,
    pub company: Company,
    pub invoice_tax: f64,
    pub invoice_reference: String,
    pub services: Vec<Service>,
    pub created_by: String,
    pub status: PaymentStatus,
}

impl Default for Invoice {
    fn default() -> Self {
        Self {
            id: rand::random::<i32>(),
            invoice_number: "0000".to_string(),
            invoice_date: "01.01.2021".to_string(),
            invoice_location: "Ljubljana".to_string(),
            service_date: "01.01.2021".to_string(),
            invoice_currency: "EUR".to_string(),
            due_date: "01.01.2021".to_string(),
            partner: Partner::default(),
            company: Company::default(),
            invoice_tax: 0.0,
            invoice_reference: "0000".to_string(),
            services: vec![Service::default()],
            created_by: "Invoicer".to_string(),
            status: PaymentStatus::Unpaid,
        }
    }
}

impl Default for InvoiceStructure {
    fn default() -> Self {
        Self {
            font_sizes: FontSizes {
                small: 9.0,
                medium: 12.0,
                large: 16.0,
            },
        }
    }
}

impl Racun {
    pub fn default() -> Self {
        Self {
            invoice: Invoice::default(),
            config: InvoiceStructure::default(),
        }
    }
}
