use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126811698: FileType = FileType {
    file_format: &FileFormat {
        id: 126_811_698,
        source_type: SourceType::Wikidata,
        name: "Booasm Compressed Archive",
        extensions: &["boo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
