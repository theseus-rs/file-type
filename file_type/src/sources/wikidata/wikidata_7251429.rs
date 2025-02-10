use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7251429: FileType = FileType {
    file_format: &FileFormat {
        id: 7_251_429,
        source_type: SourceType::Wikidata,
        name: "Protein Data Bank",
        extensions: &["pdb"],
        media_types: &["chemical/x-pdb"],
        signatures: &[],
        related_formats: &[],
    },
};
