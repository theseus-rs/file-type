use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59713556: FileType = FileType {
    file_format: &FileFormat {
        id: 59_713_556,
        source_type: SourceType::Wikidata,
        name: "Gaussian Input Data File",
        extensions: &["gjf"],
        media_types: &["chemical/x-gaussian-input"],
        signatures: &[],
        related_formats: &[],
    },
};
