use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1228359: FileType = FileType {
    file_format: &FileFormat {
        id: 1_228_359,
        source_type: SourceType::Wikidata,
        name: "Disc Description Protocol",
        extensions: &["ddp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
