use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47519807: FileType = FileType {
    file_format: &FileFormat {
        id: 47_519_807,
        source_type: SourceType::Wikidata,
        name: "Serif PagePlus Publication file format, version 4",
        extensions: &["ppp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
