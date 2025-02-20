use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111317331: FileType = FileType {
    file_format: &FileFormat {
        id: 111_317_331,
        source_type: SourceType::Wikidata,
        name: "Native Instruments Reaktor format",
        extensions: &["map"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
