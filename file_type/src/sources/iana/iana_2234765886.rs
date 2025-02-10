use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2234765886: FileType = FileType {
    file_format: &FileFormat {
        id: 2_234_765_886,
        source_type: SourceType::Iana,
        name: "encrypted",
        extensions: &[],
        media_types: &["multipart/encrypted"],
        signatures: &[],
        related_formats: &[],
    },
};
