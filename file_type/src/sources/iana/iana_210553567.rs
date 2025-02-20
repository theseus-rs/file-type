use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_210553567: FileType = FileType {
    file_format: &FileFormat {
        id: 210_553_567,
        source_type: SourceType::Iana,
        name: "vnd.oasis.opendocument.presentation-template",
        extensions: &[],
        media_types: &["application/vnd.oasis.opendocument.presentation-template"],
        signatures: &[],
        related_formats: &[],
    },
};
