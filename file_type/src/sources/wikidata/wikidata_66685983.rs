use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66685983: FileType = FileType {
    file_format: &FileFormat {
        id: 66_685_983,
        source_type: SourceType::Wikidata,
        name: "OR3",
        extensions: &["or3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
