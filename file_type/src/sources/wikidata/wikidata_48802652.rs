use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48802652: FileType = FileType {
    file_format: &FileFormat {
        id: 48_802_652,
        source_type: SourceType::Wikidata,
        name: "Aldus Freehand Drawing, version 4",
        extensions: &["fh4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
