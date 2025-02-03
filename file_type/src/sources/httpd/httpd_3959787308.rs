use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3959787308: FileFormat = FileFormat {
    id: 3_959_787_308,
    source_type: SourceType::Httpd,
    name: "dece unspecified",
    extensions: &["uvx", "uvvx"],
    media_types: &["application/vnd.dece.unspecified"],
    internal_signatures: &[],
    related_formats: &[],
};
