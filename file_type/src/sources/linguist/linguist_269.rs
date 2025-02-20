use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_269: FileType = FileType {
    file_format: &FileFormat {
        id: 269,
        source_type: SourceType::Linguist,
        name: "Oxygene",
        extensions: &["oxygene"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
