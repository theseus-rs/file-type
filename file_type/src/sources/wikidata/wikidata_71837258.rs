use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71837258: FileType = FileType {
    file_format: &FileFormat {
        id: 71_837_258,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Compressed Drawing file format",
        extensions: &["cdx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
