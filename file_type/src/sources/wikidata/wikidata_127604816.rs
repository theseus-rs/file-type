use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127604816: FileType = FileType {
    file_format: &FileFormat {
        id: 127_604_816,
        source_type: SourceType::Wikidata,
        name: "AMPL model file",
        extensions: &["mod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
