use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206851: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_851,
        source_type: SourceType::Wikidata,
        name: "Secure Pick Ax file",
        extensions: &["pax"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
