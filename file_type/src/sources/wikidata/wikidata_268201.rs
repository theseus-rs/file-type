use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_268201: FileType = FileType {
    file_format: &FileFormat {
        id: 268_201,
        source_type: SourceType::Wikidata,
        name: "Wireless Markup Language",
        extensions: &["wml"],
        media_types: &["text/vnd.wap.wml"],
        signatures: &[],
        related_formats: &[],
    },
};
