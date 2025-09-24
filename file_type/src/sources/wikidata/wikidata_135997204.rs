use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135997204: FileType = FileType {
    file_format: &FileFormat {
        id: 135_997_204,
        source_type: SourceType::Wikidata,
        name: "Brotli",
        extensions: &["br"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
