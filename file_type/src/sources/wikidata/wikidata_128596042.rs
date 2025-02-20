use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128596042: FileType = FileType {
    file_format: &FileFormat {
        id: 128_596_042,
        source_type: SourceType::Wikidata,
        name: "Aheui file format",
        extensions: &["aheui"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
