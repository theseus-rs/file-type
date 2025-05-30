use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
