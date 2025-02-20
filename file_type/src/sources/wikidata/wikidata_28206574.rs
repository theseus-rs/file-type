use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206574: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_574,
        source_type: SourceType::Wikidata,
        name: "MegaPaint BLD",
        extensions: &["bld"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
