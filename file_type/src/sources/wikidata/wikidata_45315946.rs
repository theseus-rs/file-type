use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_45315946: FileType = FileType {
    file_format: &FileFormat {
        id: 45_315_946,
        source_type: SourceType::Wikidata,
        name: "Macromedia Freehand file format, version 7",
        extensions: &["fh7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
