use der::Sequence;

/// Policy constraints extension as defined in [RFC 5280 Section 4.2.1.11].
///
/// This extension is identified by the [`PKIX_CE_POLICY_CONSTRAINTS`](constant.PKIX_CE_POLICY_CONSTRAINTS.html) OID.
///
/// ```text
/// PolicyConstraints ::= SEQUENCE {
///      requireExplicitPolicy   [0]     SkipCerts OPTIONAL,
///      inhibitPolicyMapping    [1]     SkipCerts OPTIONAL
/// }
/// ```
///
/// [RFC 5280 Section 4.2.1.11]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.11
#[derive(Clone, Debug, Eq, PartialEq, Sequence)]
#[allow(missing_docs)]
pub struct PolicyConstraints {
    #[asn1(context_specific = "0", optional = "true", tag_mode = "IMPLICIT")]
    pub require_explicit_policy: Option<u32>,

    #[asn1(context_specific = "1", optional = "true", tag_mode = "IMPLICIT")]
    pub inhibit_policy_mapping: Option<u32>,
}
