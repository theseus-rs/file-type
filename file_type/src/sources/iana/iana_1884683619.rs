use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1884683619: FileFormat = FileFormat {
    id: 1_884_683_619,
    source_type: SourceType::Iana,
    name: "pgp-signature",
    extensions: &[],
    media_types: &["application/pgp-signature"],
    internal_signatures: &[],
    related_formats: &[],
};
