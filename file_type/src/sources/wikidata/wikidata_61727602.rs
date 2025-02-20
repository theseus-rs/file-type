use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61727602: FileType = FileType {
    file_format: &FileFormat {
        id: 61_727_602,
        source_type: SourceType::Wikidata,
        name: "PrimeOCR file format, version 4.3",
        extensions: &["pro"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
