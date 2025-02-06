use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1250972188: FileFormat = FileFormat {
    id: 1_250_972_188,
    source_type: SourceType::Httpd,
    name: "scvp vp response",
    extensions: &["spp"],
    media_types: &["application/scvp-vp-response"],
    signatures: &[],
    related_formats: &[],
};
