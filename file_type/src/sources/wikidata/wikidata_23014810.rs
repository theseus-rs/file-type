use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_23014810: FileType = FileType {
    file_format: &FileFormat {
        id: 23_014_810,
        source_type: SourceType::Wikidata,
        name: "chr",
        extensions: &["chr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
