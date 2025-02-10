use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3086943397: FileType = FileType {
    file_format: &FileFormat {
        id: 3_086_943_397,
        source_type: SourceType::Iana,
        name: "vnd.oasis.opendocument.spreadsheet-template",
        extensions: &[],
        media_types: &["application/vnd.oasis.opendocument.spreadsheet-template"],
        signatures: &[],
        related_formats: &[],
    },
};
