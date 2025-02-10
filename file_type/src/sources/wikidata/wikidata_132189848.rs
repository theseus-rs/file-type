use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_132189848: FileType = FileType {
    file_format: &FileFormat {
        id: 132_189_848,
        source_type: SourceType::Wikidata,
        name: "Spanish Braille file format",
        extensions: &["bra"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
