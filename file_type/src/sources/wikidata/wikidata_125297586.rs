use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125297586: FileType = FileType {
    file_format: &FileFormat {
        id: 125_297_586,
        source_type: SourceType::Wikidata,
        name: "Scheme program source",
        extensions: &["sps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
