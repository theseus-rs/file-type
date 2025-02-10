use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4236885712: FileType = FileType {
    file_format: &FileFormat {
        id: 4_236_885_712,
        source_type: SourceType::Iana,
        name: "asc",
        extensions: &[],
        media_types: &["audio/asc"],
        signatures: &[],
        related_formats: &[],
    },
};
