use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_45315877: FileType = FileType {
    file_format: &FileFormat {
        id: 45_315_877,
        source_type: SourceType::Wikidata,
        name: "Macromedia Freehand file format, version 9",
        extensions: &["fh9"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
