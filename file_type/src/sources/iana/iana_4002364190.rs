use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4002364190: FileType = FileType {
    file_format: &FileFormat {
        id: 4_002_364_190,
        source_type: SourceType::Iana,
        name: "e57",
        extensions: &[],
        media_types: &["model/e57"],
        signatures: &[],
        related_formats: &[],
    },
};
