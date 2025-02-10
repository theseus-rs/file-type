use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127692064: FileType = FileType {
    file_format: &FileFormat {
        id: 127_692_064,
        source_type: SourceType::Wikidata,
        name: "freefem format",
        extensions: &["msh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
