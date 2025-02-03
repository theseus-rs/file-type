use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2363579130: FileFormat = FileFormat {
    id: 2_363_579_130,
    source_type: SourceType::Httpd,
    name: "dwg",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
