use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_838252715: FileType = FileType {
    file_format: &FileFormat {
        id: 838_252_715,
        source_type: SourceType::Linguist,
        name: "Ink",
        extensions: &["ink"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
