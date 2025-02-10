use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_10394822: FileType = FileType {
    file_format: &FileFormat {
        id: 10_394_822,
        source_type: SourceType::Wikidata,
        name: "ZIP archive file format, version 6.3.2",
        extensions: &["zip", "zipx"],
        media_types: &["application/zip"],
        signatures: &[],
        related_formats: &[],
    },
};
