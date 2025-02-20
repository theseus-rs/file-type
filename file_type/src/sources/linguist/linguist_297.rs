use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_297: FileType = FileType {
    file_format: &FileFormat {
        id: 297,
        source_type: SourceType::Linguist,
        name: "Protocol Buffer",
        extensions: &["proto"],
        media_types: &["text/x-protobuf"],
        signatures: &[],
        related_formats: &[],
    },
};
