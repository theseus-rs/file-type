use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29012285: FileType = FileType {
    file_format: &FileFormat {
        id: 29_012_285,
        source_type: SourceType::Wikidata,
        name: "Shockwave Flash, version 29",
        extensions: &["swf"],
        media_types: &["application/x-shockwave-flash"],
        signatures: &[],
        related_formats: &[],
    },
};
