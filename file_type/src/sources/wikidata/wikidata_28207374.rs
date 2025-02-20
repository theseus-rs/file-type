use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207374: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_374,
        source_type: SourceType::Wikidata,
        name: "Technicolor Dream COL",
        extensions: &["col"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
