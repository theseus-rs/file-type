use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3458060688: FileType = FileType {
    file_format: &FileFormat {
        id: 3_458_060_688,
        source_type: SourceType::Httpd,
        name: "winhlp",
        extensions: &["hlp"],
        media_types: &["application/winhlp"],
        signatures: &[],
        related_formats: &[],
    },
};
