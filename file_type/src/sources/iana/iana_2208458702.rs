use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2208458702: FileType = FileType {
    file_format: &FileFormat {
        id: 2_208_458_702,
        source_type: SourceType::Iana,
        name: "urc-grpsheet+xml",
        extensions: &[],
        media_types: &["application/urc-grpsheet+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
