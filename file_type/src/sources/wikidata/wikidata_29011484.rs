use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29011484: FileType = FileType {
    file_format: &FileFormat {
        id: 29_011_484,
        source_type: SourceType::Wikidata,
        name: "Shockwave Flash, version 11",
        extensions: &["swf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
