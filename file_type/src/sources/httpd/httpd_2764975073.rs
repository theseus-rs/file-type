use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2764975073: FileFormat = FileFormat {
    id: 2_764_975_073,
    source_type: SourceType::Httpd,
    name: "recordare musicxml xml",
    extensions: &["musicxml"],
    media_types: &["application/vnd.recordare.musicxml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
