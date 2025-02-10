use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_296924: FileType = FileType {
    file_format: &FileFormat {
        id: 296_924,
        source_type: SourceType::Wikidata,
        name: "ART image file format",
        extensions: &["art"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
