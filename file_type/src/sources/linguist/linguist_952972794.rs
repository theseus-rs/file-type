use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_952972794: FileType = FileType {
    file_format: &FileFormat {
        id: 952_972_794,
        source_type: SourceType::Linguist,
        name: "ZAP",
        extensions: &["xzap", "zap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
