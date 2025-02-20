use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129922769: FileType = FileType {
    file_format: &FileFormat {
        id: 129_922_769,
        source_type: SourceType::Wikidata,
        name: "Jasmin file format",
        extensions: &["j"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
