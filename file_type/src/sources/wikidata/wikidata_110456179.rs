use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110456179: FileType = FileType {
    file_format: &FileFormat {
        id: 110_456_179,
        source_type: SourceType::Wikidata,
        name: "Standard Data Format",
        extensions: &["sdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
