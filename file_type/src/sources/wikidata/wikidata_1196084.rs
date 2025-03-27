use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1196084: FileType = FileType {
    file_format: &FileFormat {
        id: 1_196_084,
        source_type: SourceType::Wikidata,
        name: "MXML",
        extensions: &["mxml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
