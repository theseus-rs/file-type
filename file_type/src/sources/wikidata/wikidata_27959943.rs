use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27959943: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_943,
        source_type: SourceType::Wikidata,
        name: "La Lossless Audio",
        extensions: &["la"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
