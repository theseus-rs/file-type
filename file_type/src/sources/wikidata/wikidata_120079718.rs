use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120079718: FileType = FileType {
    file_format: &FileFormat {
        id: 120_079_718,
        source_type: SourceType::Wikidata,
        name: "Matisse file",
        extensions: &["mat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
