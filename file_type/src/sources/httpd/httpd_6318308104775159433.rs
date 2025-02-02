use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6318308104775159433: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pkix pkipath",
    extensions: &["pkipath"],
    media_types: &["application/pkix-pkipath"],
    internal_signatures: &[],
    related_formats: &[],
};
