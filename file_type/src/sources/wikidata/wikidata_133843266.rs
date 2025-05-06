use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133843266: FileType = FileType {
    file_format: &FileFormat {
        id: 133_843_266,
        source_type: SourceType::Wikidata,
        name: "Raw Teletext file",
        extensions: &["bin"],
        media_types: &["text/x-raw-teletext"],
        signatures: &[],
        related_formats: &[],
    },
};
