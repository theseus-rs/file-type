use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61727569: FileType = FileType {
    file_format: &FileFormat {
        id: 61_727_569,
        source_type: SourceType::Wikidata,
        name: "PrimeOCR file format, version 4.2",
        extensions: &["pro"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
