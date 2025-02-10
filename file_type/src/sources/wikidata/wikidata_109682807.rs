use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109682807: FileType = FileType {
    file_format: &FileFormat {
        id: 109_682_807,
        source_type: SourceType::Wikidata,
        name: "Sinar Digital Back format",
        extensions: &["sti"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
