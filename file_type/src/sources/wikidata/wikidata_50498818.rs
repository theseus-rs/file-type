use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50498818: FileType = FileType {
    file_format: &FileFormat {
        id: 50_498_818,
        source_type: SourceType::Wikidata,
        name: "Geography Markup Language, version 3.2",
        extensions: &["gml"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
