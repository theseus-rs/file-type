use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3181687882: FileType = FileType {
    file_format: &FileFormat {
        id: 3_181_687_882,
        source_type: SourceType::Iana,
        name: "vnd.groove-tool-message",
        extensions: &[],
        media_types: &["application/vnd.groove-tool-message"],
        signatures: &[],
        related_formats: &[],
    },
};
