use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_592853203: FileType = FileType {
    file_format: &FileFormat {
        id: 592_853_203,
        source_type: SourceType::Linguist,
        name: "RouterOS Script",
        extensions: &["rsc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
