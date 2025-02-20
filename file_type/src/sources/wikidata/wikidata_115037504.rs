use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_115037504: FileType = FileType {
    file_format: &FileFormat {
        id: 115_037_504,
        source_type: SourceType::Wikidata,
        name: "Extensible Markup Language 1.1",
        extensions: &["xml"],
        media_types: &["application/xml", "text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
