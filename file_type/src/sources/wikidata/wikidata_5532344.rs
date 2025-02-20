use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5532344: FileType = FileType {
    file_format: &FileFormat {
        id: 5_532_344,
        source_type: SourceType::Wikidata,
        name: "General feature format",
        extensions: &["gff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
