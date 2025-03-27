use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112796985: FileType = FileType {
    file_format: &FileFormat {
        id: 112_796_985,
        source_type: SourceType::Wikidata,
        name: "Leica RAW Image",
        extensions: &["rwl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
