use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_572396452: FileType = FileType {
    file_format: &FileFormat {
        id: 572_396_452,
        source_type: SourceType::Iana,
        name: "entity-statement+jwt",
        extensions: &[],
        media_types: &["application/entity-statement+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
