use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130283788: FileType = FileType {
    file_format: &FileFormat {
        id: 130_283_788,
        source_type: SourceType::Wikidata,
        name: "Maxima file format",
        extensions: &["mac", "max"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
