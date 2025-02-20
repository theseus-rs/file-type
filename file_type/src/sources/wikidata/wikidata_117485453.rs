use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117485453: FileType = FileType {
    file_format: &FileFormat {
        id: 117_485_453,
        source_type: SourceType::Wikidata,
        name: "MacCaption File 2",
        extensions: &["mcc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
