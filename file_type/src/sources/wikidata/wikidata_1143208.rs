use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1143208: FileType = FileType {
    file_format: &FileFormat {
        id: 1_143_208,
        source_type: SourceType::Wikidata,
        name: "Cue sheet",
        extensions: &["cue"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
