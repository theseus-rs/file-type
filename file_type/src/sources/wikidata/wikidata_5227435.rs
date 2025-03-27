use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5227435: FileType = FileType {
    file_format: &FileFormat {
        id: 5_227_435,
        source_type: SourceType::Wikidata,
        name: "Datafork TrueType",
        extensions: &["dfont"],
        media_types: &["application/x-dfont"],
        signatures: &[],
        related_formats: &[],
    },
};
