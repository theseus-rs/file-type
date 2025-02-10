use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60614979: FileType = FileType {
    file_format: &FileFormat {
        id: 60_614_979,
        source_type: SourceType::Wikidata,
        name: "Serif DrawPlus Drawing, version 4",
        extensions: &["dpp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
