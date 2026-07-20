use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_140323677: FileType = FileType {
    file_format: &FileFormat {
        id: 140_323_677,
        source_type: SourceType::Wikidata,
        name: "Nexus",
        extensions: &["nxs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
