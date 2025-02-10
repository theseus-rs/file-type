use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_134534086: FileType = FileType {
    file_format: &FileFormat {
        id: 134_534_086,
        source_type: SourceType::Linguist,
        name: "WebAssembly Interface Type",
        extensions: &["wit"],
        media_types: &["text/x-webidl"],
        signatures: &[],
        related_formats: &[],
    },
};
