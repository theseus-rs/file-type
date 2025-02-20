use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_94279981: FileType = FileType {
    file_format: &FileFormat {
        id: 94_279_981,
        source_type: SourceType::Wikidata,
        name: "Dragon",
        extensions: &["dgn"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
