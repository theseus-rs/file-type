use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113487415: FileType = FileType {
    file_format: &FileFormat {
        id: 113_487_415,
        source_type: SourceType::Wikidata,
        name: "Djot",
        extensions: &["dj"],
        media_types: &[
            "http://www.wikidata.org/.well-known/genid/07ddce858d714e74c6f43dccd9b06352",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
