use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58103380: FileType = FileType {
    file_format: &FileFormat {
        id: 58_103_380,
        source_type: SourceType::Wikidata,
        name: "eRuby HTML document",
        extensions: &["rhtm", "rhtml"],
        media_types: &["text/html+ruby"],
        signatures: &[],
        related_formats: &[],
    },
};
