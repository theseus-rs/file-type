use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60478880: FileType = FileType {
    file_format: &FileFormat {
        id: 60_478_880,
        source_type: SourceType::Wikidata,
        name: "Serif DrawPlus Drawing, version 3",
        extensions: &["dpp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
