use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118644981: FileType = FileType {
    file_format: &FileFormat {
        id: 118_644_981,
        source_type: SourceType::Wikidata,
        name: "ISOBMFF Segment",
        extensions: &["m4s"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
