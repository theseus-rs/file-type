use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_393: FileType = FileType {
    file_format: &FileFormat {
        id: 393,
        source_type: SourceType::Linguist,
        name: "Wavefront Object",
        extensions: &["obj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
