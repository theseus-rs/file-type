use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29017306: FileType = FileType {
    file_format: &FileFormat {
        id: 29_017_306,
        source_type: SourceType::Wikidata,
        name: "Shockwave Flash, version 32",
        extensions: &["swf"],
        media_types: &["application/x-shockwave-flash"],
        signatures: &[],
        related_formats: &[],
    },
};
