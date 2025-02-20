use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50288102: FileType = FileType {
    file_format: &FileFormat {
        id: 50_288_102,
        source_type: SourceType::Wikidata,
        name: "Mathcad Document, binary file format",
        extensions: &["mcd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
