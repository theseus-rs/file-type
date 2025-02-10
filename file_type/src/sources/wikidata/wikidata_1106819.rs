use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1106819: FileType = FileType {
    file_format: &FileFormat {
        id: 1_106_819,
        source_type: SourceType::Wikidata,
        name: "CoffeeScript",
        extensions: &["coffee"],
        media_types: &["application/vnd.coffeescript", "text/coffeescript"],
        signatures: &[],
        related_formats: &[],
    },
};
