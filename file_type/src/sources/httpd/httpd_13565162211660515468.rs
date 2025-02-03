use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13565162211660515468: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pgp encrypted",
    extensions: &["pgp"],
    media_types: &["application/pgp-encrypted"],
    internal_signatures: &[],
    related_formats: &[],
};
