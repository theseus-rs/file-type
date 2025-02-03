use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1349874601: FileFormat = FileFormat {
    id: 1_349_874_601,
    source_type: SourceType::Httpd,
    name: "ecowin chart",
    extensions: &["mag"],
    media_types: &["application/vnd.ecowin.chart"],
    internal_signatures: &[],
    related_formats: &[],
};
