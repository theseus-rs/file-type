use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_270184138: FileType = FileType {
    file_format: &FileFormat {
        id: 270_184_138,
        source_type: SourceType::Linguist,
        name: "Cadence",
        extensions: &["cdc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
