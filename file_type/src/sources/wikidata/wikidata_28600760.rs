use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600760: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_760,
        source_type: SourceType::Wikidata,
        name: "Early Mind Manager XML format",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
