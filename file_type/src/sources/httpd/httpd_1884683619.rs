use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1884683619: FileFormat = FileFormat {
    id: 1_884_683_619,
    source_type: SourceType::Httpd,
    name: "pgp signature",
    extensions: &["asc", "sig"],
    media_types: &["application/pgp-signature"],
    signatures: &[],
    related_formats: &[],
};
