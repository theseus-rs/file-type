use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116884421: FileType = FileType {
    file_format: &FileFormat {
        id: 116_884_421,
        source_type: SourceType::Wikidata,
        name: "Adobe PhotoDeluxe data",
        extensions: &["pbd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
