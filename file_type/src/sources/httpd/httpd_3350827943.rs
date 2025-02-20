use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3350827943: FileType = FileType {
    file_format: &FileFormat {
        id: 3_350_827_943,
        source_type: SourceType::Httpd,
        name: "gramps xml",
        extensions: &["gramps"],
        media_types: &["application/x-gramps-xml"],
        signatures: &[],
        related_formats: &[],
    },
};
