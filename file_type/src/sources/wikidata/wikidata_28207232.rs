use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207232: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_232,
        source_type: SourceType::Wikidata,
        name: "RLA",
        extensions: &["rla"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
