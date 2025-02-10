use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61641450: FileType = FileType {
    file_format: &FileFormat {
        id: 61_641_450,
        source_type: SourceType::Wikidata,
        name: "WordPerfect for MS-DOS Document",
        extensions: &["w50", "wp", "wp5", "wpd"],
        media_types: &["application/vnd.wordperfect"],
        signatures: &[],
        related_formats: &[],
    },
};
