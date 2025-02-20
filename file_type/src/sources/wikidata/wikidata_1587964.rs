use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1587964: FileType = FileType {
    file_format: &FileFormat {
        id: 1_587_964,
        source_type: SourceType::Wikidata,
        name: "Harwell-Boeing file format",
        extensions: &["rua"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
