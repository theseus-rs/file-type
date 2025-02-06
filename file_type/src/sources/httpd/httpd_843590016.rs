use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_843590016: FileFormat = FileFormat {
    id: 843_590_016,
    source_type: SourceType::Httpd,
    name: "openxmlformats officedocument wordprocessingml document",
    extensions: &["docx"],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
    signatures: &[],
    related_formats: &[],
};
