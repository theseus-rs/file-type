use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137440805: FileType = FileType {
    file_format: &FileFormat {
        id: 137_440_805,
        source_type: SourceType::Wikidata,
        name: "Dendroscope file format",
        extensions: &["dendro"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
