use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_73019664: FileType = FileType {
    file_format: &FileFormat {
        id: 73_019_664,
        source_type: SourceType::Wikidata,
        name: "WordPerfect for MS-DOS/Windows Document, version 5.1",
        extensions: &["wp5"],
        media_types: &["application/vnd.wordperfect"],
        signatures: &[],
        related_formats: &[],
    },
};
