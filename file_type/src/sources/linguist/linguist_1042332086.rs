use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_1042332086: FileType = FileType {
    file_format: &FileFormat {
        id: 1_042_332_086,
        source_type: SourceType::Linguist,
        name: "Circom",
        extensions: &["circom"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
