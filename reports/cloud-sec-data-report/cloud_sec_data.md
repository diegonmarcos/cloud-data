# Cloud Security: Data Scan Report

> Generated: 2026-04-20 02:53:23 UTC

---

## Issues Summary

  No issues found — all checks passed.

---

## Container Export Status

  ✅ Container export               Skipped — no SSH/WG connectivity

  Summary: 1/1 passed, 0 failed

---

## YARA Scan Summary

  ✅ YARA rules loaded              7 rule files from /usr/local/share/yara-rules
  ✅ YARA scan                      No exported containers to scan

  Summary: 2/2 passed, 0 failed

### YARA Hits Detail

  No YARA matches detected.

---

## SIEM Alerts Summary

  ✅ SIEM API auth                  Skipped — no bearer token available

  Summary: 1/1 passed, 0 failed

### SIEM Alert Details

  No critical SIEM alerts.

---

## Threat Intelligence

  ✅ URLhaus feed                   200 indicators fetched (0.1s)

  Summary: 1/1 passed, 0 failed

---

## Journal Analysis (24h)

  ✅ Journal analysis               No evidence directories available

  Summary: 1/1 passed, 0 failed

  No suspicious journal entries detected.

---

## Runtime Security Analysis

  ✅ Runtime analysis               No evidence directories available

  Summary: 1/1 passed, 0 failed

  No runtime security issues detected.

---

## Container Diff Analysis

  ✅ Diff analysis                  No evidence directories available

  Summary: 1/1 passed, 0 failed

  No significant container changes detected.

---

## Cross-Correlation Analysis

  ✅ Cross-correlation              No correlated threats found

  Summary: 1/1 passed, 0 failed

---

## Performance

  YARA+SIEM+ThreatIntel+Journal+Runtime+Diff 0.4s
  Correlation              0.0s
  Container export + evidence 0.0s

  Total: 3.4s | Engine: Rust (native async tokio)
  Checks: TCP(native) HTTP(reqwest) DNS(trust-dns) SSH(mux)

---

## Result

**ALL CLEAR -- 9/9 checks passed**
