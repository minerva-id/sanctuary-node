# Security Policy

## 🔐 Reporting a Vulnerability

The Tesserax Protocol team takes security seriously. We appreciate your efforts to responsibly disclose your findings.

### 🚨 **DO NOT** open public issues for security vulnerabilities

Instead, please report security issues via:

📧 **Email**: security@tesserax.network

### What to Include

When reporting a vulnerability, please include:

1. **Description** of the vulnerability
2. **Steps to reproduce** the issue
3. **Potential impact** assessment
4. **Affected versions** (if known)
5. **Suggested fix** (if you have one)
6. **Your contact information** (for follow-up)

### Response Timeline

| Stage | Timeline |
|-------|----------|
| **Initial Response** | Within 48 hours |
| **Triage \u0026 Assessment** | Within 1 week |
| **Fix Development** | Varies by severity |
| **Public Disclosure** | After fix is deployed |

## 🏆 Responsible Security Disclosure

Tesserax Protocol values the security research community and recognizes responsible disclosure.

### Recognition Program

Security researchers who responsibly disclose vulnerabilities will receive:

- **Public Recognition**: Listed in our Security Hall of Fame
- **Contributor Credits**: Acknowledged in release notes and CHANGELOG
- **Priority Support**: Direct communication channel with core team
- **Early Access**: Beta access to new features and testnets
- **Community Role**: Special Discord/Telegram role for contributors

### Severity Classification

We classify vulnerabilities to prioritize response:

| Severity | Examples | Response Priority |
|----------|----------|-------------------|
| **Critical** | Consensus break, unauthorized minting, cryptographic breaks | Immediate (24h) |
| **High** | Signature bypass, replay attacks, vault compromise | Urgent (72h) |
| **Medium** | DoS vectors, RPC exploits, storage corruption | High (1 week) |
| **Low** | Information disclosure, minor logic errors | Normal (2 weeks) |

### Future Bug Bounty Program

We plan to launch a funded bug bounty program after:
- Mainnet launch
- Treasury funding secured
- Governance framework established

Stay tuned for announcements!

### Eligibility for Recognition

- Vulnerabilities in the latest release
- Not previously reported
- Follows responsible disclosure process
- Not result of social engineering
- Provides clear reproduction steps

### Out of Scope

- Previously disclosed vulnerabilities
- Testnet-only issues (unless critical)
- Social engineering attacks
- Physical attacks
- Issues in third-party dependencies (report to upstream)


## 🛡️ Security Best Practices

### For Users

1. **Verify Binaries**
   ```bash
   # Check release signatures
   gpg --verify tesserax-node-vX.Y.Z.tar.gz.sig
   ```

2. **Use Quantum Vault for Large Holdings**
   - Create vault with generated Dilithium keys
   - Store private keys offline (hardware wallet or paper)
   - Never share vault private keys

3. **Keep Software Updated**
   - Subscribe to security advisories
   - Update to latest releases promptly

### For Developers

1. **Follow Secure Coding Practices**
   - Use `saturating_*` operations to prevent overflow
   - Validate all user inputs
   - Never use `unwrap()` in production code
   - Use proper error handling

2. **Review Dependencies**
   ```bash
   cargo audit
   cargo deny check
   ```

3. **Test Thoroughly**
   - Write comprehensive unit tests
   - Include fuzzing tests for critical paths
   - Test edge cases and error conditions

## 🔍 Security Audit History

### Completed Audits

*No external audits completed yet*

### Planned Audits

| Component | Status | Timeline |
|-----------|--------|----------|
| **Re-ML System** | Planned | Q2 2026 |
| **Quantum Vault** | Planned | Q2 2026 |
| **Emission Pallet** | Internal Audit Complete | ✅ Jan 2026 |

## 🎯 Security Focus Areas

### 1. Cryptographic Components

- **Quantum Vault**: CRYSTALS-Dilithium implementation
- **Re-ML**: ML-DSA signature verification in SP1 zkVM
- **Consensus**: GRANDPA finality gadget

### 2. Economic Security

- **Emission Schedule**: Pre-computed table integrity
- **Fee Mechanisms**: Treasury accounting
- **Token Minting**: Authorization \u0026 supply limits

### 3. Network Security

- **P2P Layer**: DoS protection, peer validation
- **RPC Endpoints**: Rate limiting, input validation
- **EVM Compatibility**: Smart contract safety

## 🔒 Known Security Considerations

### Current Security Notes

1. **VKey Hash Configuration**
   - **Status**: Development placeholder (`[0u8; 32]`)
   - **Production Action Required**: Set actual SP1 guest VKey hash
   - **Impact**: Re-ML proof verification
   - **Timeline**: Before mainnet

2. **Emission Table Immutability**
   - **Design**: Hardcoded emission schedule
   - **Rationale**: Determinism \u0026 auditability
   - **Consideration**: No emergency adjustment capability
   - **Mitigation**: Extensive pre-launch testing

3. **Quantum Vault Assumptions**
   - **Assumption**: NIST PQC algorithms remain secure
   - **Monitoring**: Track NIST PQC standardization
   - **Fallback**: Governance-enabled algorithm migration

## 📋 Security Checklist for Contributors

Before submitting code:

- [ ] No hardcoded secrets or keys
- [ ] All user inputs validated
- [ ] Proper error handling (no unwrap in production)
- [ ] Integer overflow protection (saturating operations)
- [ ] Tests include security test cases
- [ ] Dependencies are up-to-date (`cargo audit`)
- [ ] Review cryptographic implementations carefully
- [ ] Documentation includes security considerations

## 🚀 Incident Response

### In Case of Security Incident

1. **Immediate Actions**
   - Assess severity and impact
   - Notify core team (security@tesserax.network)
   - Prepare hotfix if needed

2. **Communication**
   - Internal: Immediate notification to core devs
   - Public: After fix is available (coordinated disclosure)
   - Affected users: Direct notification if possible

3. **Resolution**
   - Deploy fix to testnet
   - Thorough testing
   - Coordinated mainnet deployment
   - Post-mortem analysis

### Emergency Contacts

| Role | Contact |
|------|---------|
| **Security Lead** | security@tesserax.network |
| **Technical Lead** | Minerva \u0026 Gemini |
| **Emergency Hotline** | security@tesserax.network |

## 📚 Security Resources

### External Resources

- [NIST Post-Quantum Cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [Substrate Security Best Practices](https://docs.substrate.io/maintain/runtime-security/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)

### Internal Documentation

- [Security Audit Report](docs/security-audit.md)
- [Quantum Vault Specification](pallets/quantum-vault/README.md)
- [Re-ML Architecture](docs/Re-ML.md)

## 🏅 Hall of Fame

Security researchers who have responsibly disclosed vulnerabilities:

*List will be updated as vulnerabilities are reported and fixed*

---

**Thank you for helping keep Tesserax Protocol secure!** 🛡️

For any security questions or concerns: **security@tesserax.network**
