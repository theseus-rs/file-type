use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205773: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_773,
        source_type: SourceType::Wikidata,
        name: "BioRad confocal image",
        extensions: &["pic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
