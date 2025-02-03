use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1968309032: FileFormat = FileFormat {
    id: 1_968_309_032,
    source_type: SourceType::Httpd,
    name: "recordare musicxml",
    extensions: &["mxl"],
    media_types: &["application/vnd.recordare.musicxml"],
    internal_signatures: &[],
    related_formats: &[],
};
