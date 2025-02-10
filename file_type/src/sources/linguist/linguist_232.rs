use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_232: FileType = FileType {
    file_format: &FileFormat {
        id: 232,
        source_type: SourceType::Linguist,
        name: "Mirah",
        extensions: &["druby", "duby", "mirah"],
        media_types: &["text/x-ruby"],
        signatures: &[],
        related_formats: &[],
    },
};
