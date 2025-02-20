use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113292085: FileType = FileType {
    file_format: &FileFormat {
        id: 113_292_085,
        source_type: SourceType::Wikidata,
        name: "InterVoice file",
        extensions: &["ivc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
