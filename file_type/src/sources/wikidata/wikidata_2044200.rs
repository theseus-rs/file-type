use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2044200: FileType = FileType {
    file_format: &FileFormat {
        id: 2_044_200,
        source_type: SourceType::Wikidata,
        name: "PICT",
        extensions: &["pct", "pict"],
        media_types: &["image/x-pict"],
        signatures: &[],
        related_formats: &[],
    },
};
