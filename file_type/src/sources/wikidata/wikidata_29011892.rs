use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29011892: FileType = FileType {
    file_format: &FileFormat {
        id: 29_011_892,
        source_type: SourceType::Wikidata,
        name: "Shockwave Flash, version 22",
        extensions: &["swf"],
        media_types: &["application/x-shockwave-flash"],
        signatures: &[],
        related_formats: &[],
    },
};
