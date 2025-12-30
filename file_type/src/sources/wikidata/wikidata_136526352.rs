use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136526352: FileType = FileType {
    file_format: &FileFormat {
        id: 136_526_352,
        source_type: SourceType::Wikidata,
        name: "Q136526352",
        extensions: &["bs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
