use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_83442984: FileType = FileType {
    file_format: &FileFormat {
        id: 83_442_984,
        source_type: SourceType::Wikidata,
        name: "Ducati Data Analyzer",
        extensions: &["dda"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
