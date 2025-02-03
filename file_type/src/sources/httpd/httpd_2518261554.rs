use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2518261554: FileFormat = FileFormat {
    id: 2_518_261_554,
    source_type: SourceType::Httpd,
    name: "f4v",
    extensions: &["f4v"],
    media_types: &["video/x-f4v"],
    internal_signatures: &[],
    related_formats: &[],
};
