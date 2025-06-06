use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
