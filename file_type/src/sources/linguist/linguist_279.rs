use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_279: FileType = FileType {
    file_format: &FileFormat {
        id: 279,
        source_type: SourceType::Linguist,
        name: "Parrot Assembly",
        extensions: &["pasm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
