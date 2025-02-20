use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_56315514: FileType = FileType {
    file_format: &FileFormat {
        id: 56_315_514,
        source_type: SourceType::Wikidata,
        name: "UML diagram",
        extensions: &["uml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
