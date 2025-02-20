use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47195633: FileType = FileType {
    file_format: &FileFormat {
        id: 47_195_633,
        source_type: SourceType::Wikidata,
        name: "AppleWorks Drawing file format, version 5",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
