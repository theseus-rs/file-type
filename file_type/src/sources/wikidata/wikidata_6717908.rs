use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6717908: FileType = FileType {
    file_format: &FileFormat {
        id: 6_717_908,
        source_type: SourceType::Wikidata,
        name: "MSSTYLES",
        extensions: &["msstyles"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
