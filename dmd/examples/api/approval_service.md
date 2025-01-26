---
dmn:
  id: D4
  requires:
    - ../loan_approval.md
---

# Approval Service API

This represents the external service that receives loan approval decisions.

## Service Endpoints

| Method | Path               | Description                     |
|--------|--------------------|---------------------------------|
| POST   | /api/approvals     | Submit new loan approval        |
| GET    | /api/approvals/{id}| Retrieve approval status        |
| PUT    | /api/approvals/{id}| Update approval status          |

## Response Codes

| Status Code | Meaning                     |
|-------------|-----------------------------|
| 200         | Success                     |
| 400         | Invalid request             |
| 401         | Unauthorized                |
| 500         | Internal server error       |

## Data Validation

| Field         | Required | Type    | Notes                          |
|---------------|----------|---------|--------------------------------|
| applicationId | Yes      | String  | Unique identifier              |
| status        | Yes      | String  | "APPROVED" or "REJECTED"       |
| timestamp     | Yes      | DateTime| Decision timestamp             |