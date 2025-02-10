use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110039243: FileType = FileType {
    file_format: &FileFormat {
        id: 110_039_243,
        source_type: SourceType::Wikidata,
        name: "XIFF (Xerox Image File Format), version 2",
        extensions: &["xif"],
        media_types: &["image/vnd.xiff"],
        signatures: &[],
        related_formats: &[],
    },
};
