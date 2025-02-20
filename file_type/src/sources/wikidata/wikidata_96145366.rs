use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_96145366: FileType = FileType {
    file_format: &FileFormat {
        id: 96_145_366,
        source_type: SourceType::Wikidata,
        name: "Wolfram Data Exchange format",
        extensions: &["wdx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
