use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7231341: FileType = FileType {
    file_format: &FileFormat {
        id: 7_231_341,
        source_type: SourceType::Wikidata,
        name: "Portable Database Image",
        extensions: &["pdi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
