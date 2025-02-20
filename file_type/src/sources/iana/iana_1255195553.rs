use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1255195553: FileType = FileType {
    file_format: &FileFormat {
        id: 1_255_195_553,
        source_type: SourceType::Iana,
        name: "3gpp",
        extensions: &[],
        media_types: &["video/3gpp"],
        signatures: &[],
        related_formats: &[],
    },
};
