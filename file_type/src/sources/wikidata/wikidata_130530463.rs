use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130530463: FileType = FileType {
    file_format: &FileFormat {
        id: 130_530_463,
        source_type: SourceType::Wikidata,
        name: "Praat script file",
        extensions: &["praat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
