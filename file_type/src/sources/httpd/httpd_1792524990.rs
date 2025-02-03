use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1792524990: FileFormat = FileFormat {
    id: 1_792_524_990,
    source_type: SourceType::Httpd,
    name: "antix game component",
    extensions: &["atx"],
    media_types: &["application/vnd.antix.game-component"],
    internal_signatures: &[],
    related_formats: &[],
};
