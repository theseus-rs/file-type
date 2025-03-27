use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29011549: FileType = FileType {
    file_format: &FileFormat {
        id: 29_011_549,
        source_type: SourceType::Wikidata,
        name: "Shockwave Flash, version 14",
        extensions: &["swf"],
        media_types: &["application/x-shockwave-flash"],
        signatures: &[],
        related_formats: &[],
    },
};
