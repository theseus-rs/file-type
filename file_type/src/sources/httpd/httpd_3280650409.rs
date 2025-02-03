use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3280650409: FileFormat = FileFormat {
    id: 3_280_650_409,
    source_type: SourceType::Httpd,
    name: "scvp cv response",
    extensions: &["scs"],
    media_types: &["application/scvp-cv-response"],
    internal_signatures: &[],
    related_formats: &[],
};
