use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2931409: FileType = FileType {
    file_format: &FileFormat {
        id: 2_931_409,
        source_type: SourceType::Wikidata,
        name: "CFD General Notation System",
        extensions: &["cgns"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
