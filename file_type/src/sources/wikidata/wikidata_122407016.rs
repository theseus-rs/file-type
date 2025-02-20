use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122407016: FileType = FileType {
    file_format: &FileFormat {
        id: 122_407_016,
        source_type: SourceType::Wikidata,
        name: "CodeWarrior CWP Project",
        extensions: &["cwp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
