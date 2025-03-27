use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_452197: FileType = FileType {
    file_format: &FileFormat {
        id: 452_197,
        source_type: SourceType::Wikidata,
        name: "Systems Biology Markup Language",
        extensions: &["sbml", "xml"],
        media_types: &["application/sbml", "application/sbml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
