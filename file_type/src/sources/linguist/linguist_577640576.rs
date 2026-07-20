use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_577640576: FileType = FileType {
    file_format: &FileFormat {
        id: 577_640_576,
        source_type: SourceType::Linguist,
        name: "FlatBuffers",
        extensions: &["fbs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
