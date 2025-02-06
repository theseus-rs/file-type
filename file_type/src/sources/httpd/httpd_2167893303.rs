use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2167893303: FileFormat = FileFormat {
    id: 2_167_893_303,
    source_type: SourceType::Httpd,
    name: "pkix pkipath",
    extensions: &["pkipath"],
    media_types: &["application/pkix-pkipath"],
    signatures: &[],
    related_formats: &[],
};
