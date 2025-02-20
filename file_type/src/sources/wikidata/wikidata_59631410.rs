use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59631410: FileType = FileType {
    file_format: &FileFormat {
        id: 59_631_410,
        source_type: SourceType::Wikidata,
        name: "Navisworks Document",
        extensions: &["nwc", "nwd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
