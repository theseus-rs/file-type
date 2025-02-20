use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113494682: FileType = FileType {
    file_format: &FileFormat {
        id: 113_494_682,
        source_type: SourceType::Wikidata,
        name: "Persuasion Auto-Template Interchange File",
        extensions: &["atf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
