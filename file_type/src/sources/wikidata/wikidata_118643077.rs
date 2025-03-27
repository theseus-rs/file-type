use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118643077: FileType = FileType {
    file_format: &FileFormat {
        id: 118_643_077,
        source_type: SourceType::Wikidata,
        name: "Ogg Audio",
        extensions: &["oga"],
        media_types: &["audio/ogg"],
        signatures: &[],
        related_formats: &[],
    },
};
