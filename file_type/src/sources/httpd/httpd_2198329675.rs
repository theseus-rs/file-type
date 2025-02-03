use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2198329675: FileFormat = FileFormat {
    id: 2_198_329_675,
    source_type: SourceType::Httpd,
    name: "caf",
    extensions: &["caf"],
    media_types: &["audio/x-caf"],
    internal_signatures: &[],
    related_formats: &[],
};
