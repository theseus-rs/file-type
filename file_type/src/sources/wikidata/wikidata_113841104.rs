use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113841104: FileType = FileType {
    file_format: &FileFormat {
        id: 113_841_104,
        source_type: SourceType::Wikidata,
        name: "Medi@Show Film File",
        extensions: &["flm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
