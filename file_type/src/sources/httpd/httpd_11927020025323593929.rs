use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11927020025323593929: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "envoy",
    extensions: &["evy"],
    media_types: &["application/x-envoy"],
    internal_signatures: &[],
    related_formats: &[],
};
