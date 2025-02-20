use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_109624286: FileType = FileType {
    file_format: &FileFormat {
        id: 109_624_286,
        source_type: SourceType::Wikidata,
        name: "WebPlus Essentials Site",
        extensions: &["wpp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
