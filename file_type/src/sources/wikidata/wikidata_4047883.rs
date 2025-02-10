use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_4047883: FileType = FileType {
    file_format: &FileFormat {
        id: 4_047_883,
        source_type: SourceType::Wikidata,
        name: "long-term prediction",
        extensions: &["gsm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
