use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111432169: FileType = FileType {
    file_format: &FileFormat {
        id: 111_432_169,
        source_type: SourceType::Wikidata,
        name: "Hypertext Template",
        extensions: &["htt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
