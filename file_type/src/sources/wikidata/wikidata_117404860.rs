use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117404860: FileType = FileType {
    file_format: &FileFormat {
        id: 117_404_860,
        source_type: SourceType::Wikidata,
        name: "VHDL Output File",
        extensions: &["vho"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
