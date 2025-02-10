use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_357046146: FileType = FileType {
    file_format: &FileFormat {
        id: 357_046_146,
        source_type: SourceType::Linguist,
        name: "Closure Templates",
        extensions: &["soy"],
        media_types: &["text/x-soy"],
        signatures: &[],
        related_formats: &[],
    },
};
