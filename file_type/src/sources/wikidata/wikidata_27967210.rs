use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967210: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_210,
        source_type: SourceType::Wikidata,
        name: "Poly Tracker module",
        extensions: &["ptm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
