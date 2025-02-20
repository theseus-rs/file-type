use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100297628: FileType = FileType {
    file_format: &FileFormat {
        id: 100_297_628,
        source_type: SourceType::Wikidata,
        name: "Flow Charting file format, version 3",
        extensions: &["fcd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
