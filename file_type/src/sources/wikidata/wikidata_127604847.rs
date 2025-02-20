use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127604847: FileType = FileType {
    file_format: &FileFormat {
        id: 127_604_847,
        source_type: SourceType::Wikidata,
        name: "AMPL data file",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
