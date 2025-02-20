use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118315834: FileType = FileType {
    file_format: &FileFormat {
        id: 118_315_834,
        source_type: SourceType::Wikidata,
        name: "FotoSlate 4.0 Project",
        extensions: &["plp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
