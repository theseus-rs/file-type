use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60628185: FileType = FileType {
    file_format: &FileFormat {
        id: 60_628_185,
        source_type: SourceType::Wikidata,
        name: "Wordperfect Secondary File, version 5",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
