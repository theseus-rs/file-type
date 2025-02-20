use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4266494892: FileType = FileType {
    file_format: &FileFormat {
        id: 4_266_494_892,
        source_type: SourceType::Iana,
        name: "rls-services+xml",
        extensions: &[],
        media_types: &["application/rls-services+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
