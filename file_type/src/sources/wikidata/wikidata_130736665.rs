use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130736665: FileType = FileType {
    file_format: &FileFormat {
        id: 130_736_665,
        source_type: SourceType::Wikidata,
        name: "Savi source code file",
        extensions: &["savi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
