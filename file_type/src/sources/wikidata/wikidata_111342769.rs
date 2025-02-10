use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111342769: FileType = FileType {
    file_format: &FileFormat {
        id: 111_342_769,
        source_type: SourceType::Wikidata,
        name: "Signed word (16-bit) data - Little Endian",
        extensions: &["sw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
