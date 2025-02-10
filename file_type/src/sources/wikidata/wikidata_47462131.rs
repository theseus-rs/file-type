use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47462131: FileType = FileType {
    file_format: &FileFormat {
        id: 47_462_131,
        source_type: SourceType::Wikidata,
        name: "Caligari TrueSpace file format (ASCII)",
        extensions: &["cob", "scn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
