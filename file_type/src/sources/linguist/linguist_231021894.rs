use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_231021894: FileType = FileType {
    file_format: &FileFormat {
        id: 231_021_894,
        source_type: SourceType::Linguist,
        name: "Hosts File",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
