use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2028789183123440400: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "crick clicker palette",
    extensions: &["clkp"],
    media_types: &["application/vnd.crick.clicker.palette"],
    internal_signatures: &[],
    related_formats: &[],
};
