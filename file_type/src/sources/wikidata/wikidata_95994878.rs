use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_95994878: FileType = FileType {
    file_format: &FileFormat {
        id: 95_994_878,
        source_type: SourceType::Wikidata,
        name: "Canadian digital elevation data format",
        extensions: &["dem"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
