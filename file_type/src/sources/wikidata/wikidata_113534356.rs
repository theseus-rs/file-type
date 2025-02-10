use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113534356: FileType = FileType {
    file_format: &FileFormat {
        id: 113_534_356,
        source_type: SourceType::Wikidata,
        name: "Pro Tools Session File",
        extensions: &["ptx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
