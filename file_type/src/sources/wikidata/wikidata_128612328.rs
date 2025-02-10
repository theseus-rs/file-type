use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128612328: FileType = FileType {
    file_format: &FileFormat {
        id: 128_612_328,
        source_type: SourceType::Wikidata,
        name: "Arturo file format",
        extensions: &["art"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
