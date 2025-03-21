use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133452018: FileType = FileType {
    file_format: &FileFormat {
        id: 133_452_018,
        source_type: SourceType::Wikidata,
        name: "brotli Archive",
        extensions: &["br"],
        media_types: &["application/x-br"],
        signatures: &[],
        related_formats: &[],
    },
};
