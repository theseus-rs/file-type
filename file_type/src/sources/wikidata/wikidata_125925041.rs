use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125925041: FileType = FileType {
    file_format: &FileFormat {
        id: 125_925_041,
        source_type: SourceType::Wikidata,
        name: "Papyrus Document Template",
        extensions: &["pav"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
