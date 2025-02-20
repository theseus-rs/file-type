use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113494624: FileType = FileType {
    file_format: &FileFormat {
        id: 113_494_624,
        source_type: SourceType::Wikidata,
        name: "Persuasion Presentation Interchange File",
        extensions: &["prf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
