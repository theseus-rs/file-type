use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600764: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_764,
        source_type: SourceType::Wikidata,
        name: "Electronic Arts MOV",
        extensions: &["mov"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
