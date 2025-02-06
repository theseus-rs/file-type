use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4226246988: FileFormat = FileFormat {
    id: 4_226_246_988,
    source_type: SourceType::Httpd,
    name: "envoy",
    extensions: &["evy"],
    media_types: &["application/x-envoy"],
    signatures: &[],
    related_formats: &[],
};
