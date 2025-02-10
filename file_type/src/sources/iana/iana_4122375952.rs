use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4122375952: FileType = FileType {
    file_format: &FileFormat {
        id: 4_122_375_952,
        source_type: SourceType::Iana,
        name: "DV",
        extensions: &[],
        media_types: &["audio/DV"],
        signatures: &[],
        related_formats: &[],
    },
};
