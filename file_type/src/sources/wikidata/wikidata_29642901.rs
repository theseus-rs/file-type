use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29642901: FileType = FileType {
    file_format: &FileFormat {
        id: 29_642_901,
        source_type: SourceType::Wikidata,
        name: "C header file",
        extensions: &["h", "hpp", "hxx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
