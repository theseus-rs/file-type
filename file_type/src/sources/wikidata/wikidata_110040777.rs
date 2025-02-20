use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110040777: FileType = FileType {
    file_format: &FileFormat {
        id: 110_040_777,
        source_type: SourceType::Wikidata,
        name: "Harvard Graphics Presentation, version 4",
        extensions: &["pr4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
