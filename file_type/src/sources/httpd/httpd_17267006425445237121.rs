use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17267006425445237121: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pkix crl",
    extensions: &["crl"],
    media_types: &["application/pkix-crl"],
    internal_signatures: &[],
    related_formats: &[],
};
