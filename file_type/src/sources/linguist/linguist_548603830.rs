use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_548603830: FileType = FileType {
    file_format: &FileFormat {
        id: 548_603_830,
        source_type: SourceType::Linguist,
        name: "Langium",
        extensions: &["langium"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
