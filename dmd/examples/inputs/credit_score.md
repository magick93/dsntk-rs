---
dmn:
  id: D3
  outputs:
    - ../loan_approval.md
---

# Credit Score Input

This represents the credit score data provided by credit bureaus.

## Credit Score Ranges

| Score Range | Rating        |
|-------------|---------------|
| 800-850     | Excellent     |
| 740-799     | Very Good     |
| 670-739     | Good          |
| 580-669     | Fair          |
| 300-579     | Poor          |

## Data Validation Rules

| Condition               | Valid |
|-------------------------|-------|
| Score between 300-850   | Yes   |
| Score outside 300-850   | No    |
| Data source verified    | Yes   |
| Data source unverified  | No    |