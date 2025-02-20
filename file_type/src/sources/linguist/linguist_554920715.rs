use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_554920715: FileType = FileType {
    file_format: &FileFormat {
        id: 554_920_715,
        source_type: SourceType::Linguist,
        name: "SSH Config",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
