use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66219660: FileType = FileType {
    file_format: &FileFormat {
        id: 66_219_660,
        source_type: SourceType::Wikidata,
        name: "shtml",
        extensions: &["sht", "shtm", "shtml", "stm"],
        media_types: &["text/x-server-parsed-html", "text/x-server-parsed-html3"],
        signatures: &[],
        related_formats: &[],
    },
};
