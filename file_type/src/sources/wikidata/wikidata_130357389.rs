use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130357389: FileType = FileType {
    file_format: &FileFormat {
        id: 130_357_389,
        source_type: SourceType::Wikidata,
        name: "MOOCode file format",
        extensions: &["moo"],
        media_types: &["text/x-moocode"],
        signatures: &[],
        related_formats: &[],
    },
};
