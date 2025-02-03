use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3458060688: FileFormat = FileFormat {
    id: 3_458_060_688,
    source_type: SourceType::Httpd,
    name: "winhlp",
    extensions: &["hlp"],
    media_types: &["application/winhlp"],
    internal_signatures: &[],
    related_formats: &[],
};
