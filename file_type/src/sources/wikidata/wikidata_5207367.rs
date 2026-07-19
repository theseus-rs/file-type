use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5207367: FileType = FileType {
    file_format: &FileFormat {
        id: 5_207_367,
        source_type: SourceType::Wikidata,
        name: "Daala",
        extensions: &["ogv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
