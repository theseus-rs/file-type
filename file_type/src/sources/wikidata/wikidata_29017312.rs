use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29017312: FileType = FileType {
    file_format: &FileFormat {
        id: 29_017_312,
        source_type: SourceType::Wikidata,
        name: "Shockwave Flash, version 35",
        extensions: &["swf"],
        media_types: &["application/x-shockwave-flash"],
        signatures: &[],
        related_formats: &[],
    },
};
