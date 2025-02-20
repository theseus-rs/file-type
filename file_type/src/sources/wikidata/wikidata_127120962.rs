use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127120962: FileType = FileType {
    file_format: &FileFormat {
        id: 127_120_962,
        source_type: SourceType::Wikidata,
        name: "Matrix Market file format",
        extensions: &["mm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
