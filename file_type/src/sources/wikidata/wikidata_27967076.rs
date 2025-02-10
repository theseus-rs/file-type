use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967076: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_076,
        source_type: SourceType::Wikidata,
        name: "Audio Sculpture",
        extensions: &["adsc", "as"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
