use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_277: FileType = FileType {
    file_format: &FileFormat {
        id: 277,
        source_type: SourceType::Linguist,
        name: "Papyrus",
        extensions: &["psc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
