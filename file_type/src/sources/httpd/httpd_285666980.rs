use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_285666980: FileFormat = FileFormat {
    id: 285_666_980,
    source_type: SourceType::Httpd,
    name: "glulx",
    extensions: &["ulx"],
    media_types: &["application/x-glulx"],
    internal_signatures: &[],
    related_formats: &[],
};
