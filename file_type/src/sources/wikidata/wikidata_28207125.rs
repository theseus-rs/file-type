use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207125: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_125,
        source_type: SourceType::Wikidata,
        name: "The New Print Shop Graphics file",
        extensions: &["pog"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
