use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1213743: FileType = FileType {
    file_format: &FileFormat {
        id: 1_213_743,
        source_type: SourceType::Wikidata,
        name: "NRG",
        extensions: &["nrg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
