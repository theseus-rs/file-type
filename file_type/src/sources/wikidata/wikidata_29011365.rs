use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29011365: FileType = FileType {
    file_format: &FileFormat {
        id: 29_011_365,
        source_type: SourceType::Wikidata,
        name: "Shockwave Flash, version 10",
        extensions: &["swf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
