use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62484348: FileType = FileType {
    file_format: &FileFormat {
        id: 62_484_348,
        source_type: SourceType::Wikidata,
        name: "AccessData Custom Content Image",
        extensions: &["ad1", "ad2", "ad3", "ad4", "ad5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
