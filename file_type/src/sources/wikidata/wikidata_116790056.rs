use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116790056: FileType = FileType {
    file_format: &FileFormat {
        id: 116_790_056,
        source_type: SourceType::Wikidata,
        name: "WordPad Document",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
