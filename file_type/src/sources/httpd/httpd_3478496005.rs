use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3478496005: FileFormat = FileFormat {
    id: 3_478_496_005,
    source_type: SourceType::Httpd,
    name: "astraea software iota",
    extensions: &["iota"],
    media_types: &["application/vnd.astraea-software.iota"],
    internal_signatures: &[],
    related_formats: &[],
};
