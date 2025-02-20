use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_538732839: FileType = FileType {
    file_format: &FileFormat {
        id: 538_732_839,
        source_type: SourceType::Linguist,
        name: "Readline Config",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
