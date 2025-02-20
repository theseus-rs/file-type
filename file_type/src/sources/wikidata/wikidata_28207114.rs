use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207114: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_114,
        source_type: SourceType::Wikidata,
        name: "The New Print Shop Names file",
        extensions: &["pnm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
