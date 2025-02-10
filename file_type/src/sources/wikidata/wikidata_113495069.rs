use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113495069: FileType = FileType {
    file_format: &FileFormat {
        id: 113_495_069,
        source_type: SourceType::Wikidata,
        name: "Calc602 Macro File",
        extensions: &["mc6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
