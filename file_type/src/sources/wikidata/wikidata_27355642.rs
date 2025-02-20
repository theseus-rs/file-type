use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27355642: FileType = FileType {
    file_format: &FileFormat {
        id: 27_355_642,
        source_type: SourceType::Wikidata,
        name: "ADRG Source File",
        extensions: &["sou"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
