use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_93275504: FileType = FileType {
    file_format: &FileFormat {
        id: 93_275_504,
        source_type: SourceType::Wikidata,
        name: "Procreate",
        extensions: &["procreate"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
