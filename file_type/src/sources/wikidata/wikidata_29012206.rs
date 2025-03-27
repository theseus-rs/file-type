use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29012206: FileType = FileType {
    file_format: &FileFormat {
        id: 29_012_206,
        source_type: SourceType::Wikidata,
        name: "Shockwave Flash, version 27",
        extensions: &["swf"],
        media_types: &["application/x-shockwave-flash"],
        signatures: &[],
        related_formats: &[],
    },
};
