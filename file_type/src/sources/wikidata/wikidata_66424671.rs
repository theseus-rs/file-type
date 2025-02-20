use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66424671: FileType = FileType {
    file_format: &FileFormat {
        id: 66_424_671,
        source_type: SourceType::Wikidata,
        name: "WordPerfect macro file format",
        extensions: &["wcm", "wpm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
