use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67383890: FileType = FileType {
    file_format: &FileFormat {
        id: 67_383_890,
        source_type: SourceType::Wikidata,
        name: "Source Engine Compiled AI Nodegraph",
        extensions: &["ain"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
