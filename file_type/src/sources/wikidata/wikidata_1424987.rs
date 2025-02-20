use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1424987: FileType = FileType {
    file_format: &FileFormat {
        id: 1_424_987,
        source_type: SourceType::Wikidata,
        name: "Notation3",
        extensions: &["n3"],
        media_types: &["text/n3"],
        signatures: &[],
        related_formats: &[],
    },
};
