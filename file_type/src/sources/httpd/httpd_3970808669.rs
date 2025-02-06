use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3970808669: FileFormat = FileFormat {
    id: 3_970_808_669,
    source_type: SourceType::Httpd,
    name: "pkix crl",
    extensions: &["crl"],
    media_types: &["application/pkix-crl"],
    signatures: &[],
    related_formats: &[],
};
