use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_466769: FileType = FileType {
    file_format: &FileFormat {
        id: 466_769,
        source_type: SourceType::Wikidata,
        name: "simplified molecular input line entry specification",
        extensions: &["smi", "smiles"],
        media_types: &["chemical/x-daylight-smiles"],
        signatures: &[],
        related_formats: &[],
    },
};
