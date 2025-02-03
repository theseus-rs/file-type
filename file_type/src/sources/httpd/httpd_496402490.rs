use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_496402490: FileFormat = FileFormat {
    id: 496_402_490,
    source_type: SourceType::Httpd,
    name: "dece audio",
    extensions: &["uva", "uvva"],
    media_types: &["audio/vnd.dece.audio"],
    internal_signatures: &[],
    related_formats: &[],
};
