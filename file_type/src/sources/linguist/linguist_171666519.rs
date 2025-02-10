use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_171666519: FileType = FileType {
    file_format: &FileFormat {
        id: 171_666_519,
        source_type: SourceType::Linguist,
        name: "NASL",
        extensions: &["inc", "nasl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
