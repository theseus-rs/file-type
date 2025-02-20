use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113487224: FileType = FileType {
    file_format: &FileFormat {
        id: 113_487_224,
        source_type: SourceType::Wikidata,
        name: "Persuasion Player File 3",
        extensions: &["ppf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
