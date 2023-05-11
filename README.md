# seiber
A library implementing multiple cybersecurity tools in rust.

### Tools to be implemented:
- [ ] Common Platform Enumeration (CPE) Version 2.3 including:
    - [ ] CPE Naming Specification [[NISTIR 7695]](https://doi.org/10.6028/NIST.IR.7695)
    - [ ] CPE Name Matching Specification [[NISTIR 7696]](https://doi.org/10.6028/NIST.IR.7696)
    - [ ] CPE Dictionary Specification [[NISTIR 7697]](https://doi.org/10.6028/NIST.IR.7697)
    - [ ] CPE Applicability Language Specification [[NISTIR 7698]](https://doi.org/10.6028/NIST.IR.7698)
- [ ] [Common Vulnerability Enumeration (CVE) JSON 5.0](https://cveproject.github.io/cve-schema/schema/v5.0/docs/)
- [ ] Common Vulnerability Scoring System (CVSS) Version 3.1
- [ ] National Vulnerability Database API 2.0

### Wishlist
 - [ ] [Common Weakness Enumeration (CWE) Version 4.x](https://cwe.mitre.org/)
 - [ ] [Known Exploited Vulnerabilities Catalog](https://www.cisa.gov/known-exploited-vulnerabilities-catalog)
 - [ ] [Common Configuration Enumeration (CCE)](https://ncp.nist.gov/cce)
 - [ ] [Security Content Automation Protocol (SCAP)](https://csrc.nist.gov/Projects/Security-Content-Automation-Protocol/)
 - [ ] Software Identification (SWID) Tag [ISO/IEC 19770-2:2015]
 - [ ] [Concise SWID (CoSWID) Tag](https://datatracker.ietf.org/doc/draft-ietf-sacm-coswid/24/)
 - [ ] [Common Attack Pattern Enumeration and Classification (CAPEC™)](https://capec.mitre.org/)
 - [ ] [MITRE ATT&CK® Version 13](https://attack.mitre.org/)


### Project Goals
This project aims to:
 - Use idiomatic rust.
 - Use only safe rust.
 - Have minimal dependencies.
 - Be able to claim compliance with the CPE standards. 
 
## Disclaimers & Trademarks
This library uses data from the [National Vulnerability Database (NVD)](https://nvd.nist.gov/) API but is not endorsed or certified by the NVD.

CPE, CAPEC & ATT&CK are trademarks of The MITRE Corporation, with which seiber has no affiliation.