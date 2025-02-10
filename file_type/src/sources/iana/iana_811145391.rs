use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_811145391: FileType = FileType {
    file_format: &FileFormat {
        id: 811_145_391,
        source_type: SourceType::Iana,
        name: "dsr-es202212",
        extensions: &[],
        media_types: &["audio/dsr-es202212"],
        signatures: &[],
        related_formats: &[],
    },
};
