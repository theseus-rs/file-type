use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29011358: FileType = FileType {
    file_format: &FileFormat {
        id: 29_011_358,
        source_type: SourceType::Wikidata,
        name: "Shockwave Flash, version 7",
        extensions: &["swf"],
        media_types: &["application/x-shockwave-flash"],
        signatures: &[],
        related_formats: &[],
    },
};
