use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_5301941: FileType = FileType {
    file_format: &FileFormat {
        id: 5_301_941,
        source_type: SourceType::Iana,
        name: "vnd.faf+yaml",
        extensions: &[],
        media_types: &["application/vnd.faf+yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
