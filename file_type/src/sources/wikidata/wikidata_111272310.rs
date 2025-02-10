use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111272310: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_310,
        source_type: SourceType::Wikidata,
        name: "Ensoniq SQ1/SQ2/KS32 instrument file",
        extensions: &["efq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
