use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129498387: FileType = FileType {
    file_format: &FileFormat {
        id: 129_498_387,
        source_type: SourceType::Wikidata,
        name: "Haml file format",
        extensions: &["haml"],
        media_types: &["text/x-haml"],
        signatures: &[],
        related_formats: &[],
    },
};
