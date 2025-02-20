use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_72961170: FileType = FileType {
    file_format: &FileFormat {
        id: 72_961_170,
        source_type: SourceType::Wikidata,
        name: "Prescription Drug Event format",
        extensions: &["pde"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
