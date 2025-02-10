use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100377205: FileType = FileType {
    file_format: &FileFormat {
        id: 100_377_205,
        source_type: SourceType::Wikidata,
        name: "WordPerfect 4.2 Encrypted Document",
        extensions: &["wp"],
        media_types: &["application/vnd.wordperfect"],
        signatures: &[],
        related_formats: &[],
    },
};
