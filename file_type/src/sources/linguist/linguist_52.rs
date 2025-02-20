use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_52: FileType = FileType {
    file_format: &FileFormat {
        id: 52,
        source_type: SourceType::Linguist,
        name: "Cap'n Proto",
        extensions: &["capnp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
