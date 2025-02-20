use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113584320: FileType = FileType {
    file_format: &FileFormat {
        id: 113_584_320,
        source_type: SourceType::Wikidata,
        name: "Viscosity file",
        extensions: &["vsc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
