use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3386827006: FileType = FileType {
    file_format: &FileFormat {
        id: 3_386_827_006,
        source_type: SourceType::Iana,
        name: "ujcs+json",
        extensions: &[],
        media_types: &["application/ujcs+json"],
        signatures: &[],
        related_formats: &[],
    },
};
