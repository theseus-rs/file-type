use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2070717028: FileType = FileType {
    file_format: &FileFormat {
        id: 2_070_717_028,
        source_type: SourceType::Iana,
        name: "javascript",
        extensions: &[],
        media_types: &["text/javascript"],
        signatures: &[],
        related_formats: &[],
    },
};
