use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113162258: FileType = FileType {
    file_format: &FileFormat {
        id: 113_162_258,
        source_type: SourceType::Wikidata,
        name: "MyMailManager File",
        extensions: &["mml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
