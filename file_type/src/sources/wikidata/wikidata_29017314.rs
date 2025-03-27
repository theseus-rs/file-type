use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29017314: FileType = FileType {
    file_format: &FileFormat {
        id: 29_017_314,
        source_type: SourceType::Wikidata,
        name: "Shockwave Flash, version 36",
        extensions: &["swf"],
        media_types: &["application/x-shockwave-flash"],
        signatures: &[],
        related_formats: &[],
    },
};
