---
dmn:
  id: D1
  requires:
    - checks/income.md#thresholds
    - inputs/credit_score.md
  outputs:
    - api/approval_service.md
---

# Loan Approval Decision

This decision determines whether a loan application should be approved based on income and credit score.

## Decision Rules

| Income ≥ | Credit Score ≥ | Outcome  |
|----------|----------------|----------|
| 50,000   | 700            | Approved |
| 30,000   | 750            | Approved |
| 50,000   | 650            | Rejected |
| 30,000   | 700            | Rejected |

## Dependencies
- Requires [Income Check](#income-check) for income verification
- Requires [Credit Score](#credit-score) from credit bureau