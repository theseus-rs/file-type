use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62664770: FileType = FileType {
    file_format: &FileFormat {
        id: 62_664_770,
        source_type: SourceType::Wikidata,
        name: "WordPerfect for MS-DOS/Windows Document file format, version 6",
        extensions: &["doc", "w60", "wp", "wp6", "wpd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
