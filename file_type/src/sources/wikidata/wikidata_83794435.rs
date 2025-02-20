use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_83794435: FileType = FileType {
    file_format: &FileFormat {
        id: 83_794_435,
        source_type: SourceType::Wikidata,
        name: "EclipseCrossword Word List File",
        extensions: &["ewl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
