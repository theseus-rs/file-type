use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122407850: FileType = FileType {
    file_format: &FileFormat {
        id: 122_407_850,
        source_type: SourceType::Wikidata,
        name: "x86 Symbol File",
        extensions: &["isym"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
