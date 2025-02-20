use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117813268: FileType = FileType {
    file_format: &FileFormat {
        id: 117_813_268,
        source_type: SourceType::Wikidata,
        name: "Meter Data",
        extensions: &["dta"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
