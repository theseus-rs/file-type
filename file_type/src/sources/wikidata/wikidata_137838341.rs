use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137838341: FileType = FileType {
    file_format: &FileFormat {
        id: 137_838_341,
        source_type: SourceType::Wikidata,
        name: "Mozilla lang file",
        extensions: &["lang"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
