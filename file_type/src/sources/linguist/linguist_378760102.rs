use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_378760102: FileType = FileType {
    file_format: &FileFormat {
        id: 378_760_102,
        source_type: SourceType::Linguist,
        name: "YASnippet",
        extensions: &["yasnippet"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
