use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10735678142131287808: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pgp signature",
    extensions: &["asc", "sig"],
    media_types: &["application/pgp-signature"],
    internal_signatures: &[],
    related_formats: &[],
};
