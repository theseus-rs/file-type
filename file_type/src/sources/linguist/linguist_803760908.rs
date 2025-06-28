use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_803760908: FileType = FileType {
    file_format: &FileFormat {
        id: 803_760_908,
        source_type: SourceType::Linguist,
        name: "Zmodel",
        extensions: &["zmodel"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
