use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_1031374237: FileType = FileType {
    file_format: &FileFormat {
        id: 1_031_374_237,
        source_type: SourceType::Linguist,
        name: "RPC",
        extensions: &["x"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
