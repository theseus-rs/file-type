use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205626: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_626,
        source_type: SourceType::Wikidata,
        name: "Sun icon",
        extensions: &["ico", "icon"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
