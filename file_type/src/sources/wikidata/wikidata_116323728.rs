use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116323728: FileType = FileType {
    file_format: &FileFormat {
        id: 116_323_728,
        source_type: SourceType::Wikidata,
        name: "Photosuite Album File",
        extensions: &["pza"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
