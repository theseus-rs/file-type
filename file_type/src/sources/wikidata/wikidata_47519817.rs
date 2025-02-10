use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47519817: FileType = FileType {
    file_format: &FileFormat {
        id: 47_519_817,
        source_type: SourceType::Wikidata,
        name: "Serif PagePlus Publication file format, version 5",
        extensions: &["ppp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
