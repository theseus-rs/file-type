use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2126098822: FileFormat = FileFormat {
    id: 2_126_098_822,
    source_type: SourceType::Httpd,
    name: "omdoc xml",
    extensions: &["omdoc"],
    media_types: &["application/omdoc+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
