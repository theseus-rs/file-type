use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47012492: FileType = FileType {
    file_format: &FileFormat {
        id: 47_012_492,
        source_type: SourceType::Wikidata,
        name: "Nota Bene Text file format",
        extensions: &["nb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
