use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47462143: FileType = FileType {
    file_format: &FileFormat {
        id: 47_462_143,
        source_type: SourceType::Wikidata,
        name: "Caligari TrueSpace file format",
        extensions: &["cob", "scn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
