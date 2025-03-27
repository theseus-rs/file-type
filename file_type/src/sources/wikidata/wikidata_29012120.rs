use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29012120: FileType = FileType {
    file_format: &FileFormat {
        id: 29_012_120,
        source_type: SourceType::Wikidata,
        name: "Shockwave Flash, version 25",
        extensions: &["swf"],
        media_types: &["application/x-shockwave-flash"],
        signatures: &[],
        related_formats: &[],
    },
};
