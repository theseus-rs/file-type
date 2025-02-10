use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_45315974: FileType = FileType {
    file_format: &FileFormat {
        id: 45_315_974,
        source_type: SourceType::Wikidata,
        name: "Macromedia Freehand MX file format, version 11",
        extensions: &["fh11"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
