use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133844474: FileType = FileType {
    file_format: &FileFormat {
        id: 133_844_474,
        source_type: SourceType::Wikidata,
        name: "Softel Teletext file",
        extensions: &["ep1"],
        media_types: &["text/x-softel-teletext"],
        signatures: &[],
        related_formats: &[],
    },
};
