---
dmn:
  id: D2
  outputs:
    - ../loan_approval.md
---

# Income Verification

This decision verifies the applicant's income against minimum thresholds.

## Income Thresholds

| Income Source | Minimum Annual Income |
|---------------|-----------------------|
| Employment    | 30,000               |
| Self-Employed | 40,000               |
| Investments   | 50,000               |

## Rules

| Income Source | Income ≥ | Verified |
|---------------|----------|----------|
| Employment    | 30,000   | Yes      |
| Self-Employed | 40,000   | Yes      |
| Investments   | 50,000   | Yes      |
| Any           | Below    | No       |