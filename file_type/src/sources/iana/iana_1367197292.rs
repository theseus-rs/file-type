use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1367197292: FileFormat = FileFormat {
    id: 1_367_197_292,
    source_type: SourceType::Iana,
    name: "oauth-authz-req+jwt",
    extensions: &[],
    media_types: &["application/oauth-authz-req+jwt"],
    internal_signatures: &[],
    related_formats: &[],
};
