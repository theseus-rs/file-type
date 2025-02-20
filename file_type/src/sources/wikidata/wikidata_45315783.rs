use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_45315783: FileType = FileType {
    file_format: &FileFormat {
        id: 45_315_783,
        source_type: SourceType::Wikidata,
        name: "Macromedia Freehand file format, version 5",
        extensions: &["fh5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
