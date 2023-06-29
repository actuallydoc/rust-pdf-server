# Simple Rust Rocket Server for PDF generation

## Usage

```bash
$ Install the latest Rust toolchain and clone the repo
$ cargo run
```

## Endpoints

### POST /invoice - Post endpoint to create an invoice the schema of the invoice is provided below
### GET /invoice/:id - Get the invoice by id (the server will return a json response with the invoice data)

## Upcoming features
### - [] Add a database to store the invoices
### - [] Add a frontend to create invoices
### - [] Return a pdf file blob with the invoice json
# Schema structure
### You can use the following schema to create an invoice or you can edit the schema for your application

```json
{
	"invoice": {
		"id": -878490253,
		"invoiceNumber": "1",
		"invoiceDate": "01.01.2021",
		"invoiceLocation": "test",
		"serviceDate": "01.01.2021",
		"invoiceCurrency": "EUR",
		"dueDate": "01.01.2021",
		"partner": {
			"id": 1234,
			"partnerName": "test test",
			"partnerAddress": "test 33",
			"partnerPostalCode": "test, test",
			"partnerVatId": "00000000",
			"isVatPayer": false
		},
		"company": {
			"id": 123,
			"companyCurrency": "€",
			"companyName": "test, s.p.",
			"companyAddress": "test test 15",
			"companyPostalCode": "test test",
			"companyBankname": "test d.d.",
			"companyVatId": "test",
			"companyIban": "test test",
			"companySwift": "test",
			"companyRegistrationNumber": "test",
			"companyPhone": "test",
			"companySignature": null,
			"companySignaturePath": null,
			"companyVatRate": 0.0,
			"companyBusinessRegisteredAt": "test"
		},
		"invoiceTax": 0.0,
		"invoiceReference": "0000",
		"services": [
			{
				"id": 1247199806,
				"serviceName": "test",
				"serviceQuantity": 1,
				"servicePrice": 1.0
			}
		],
		"createdBy": "Invoicer",
		"status": "Unpaid"
	},
	"config": {
		"fontSizes": {
			"small": 9.0,
			"medium": 12.0,
			"large": 16.0
		}
	}
} 
```