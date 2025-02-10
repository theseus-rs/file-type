use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128917606: FileType = FileType {
    file_format: &FileFormat {
        id: 128_917_606,
        source_type: SourceType::Wikidata,
        name: "Earl Grey source code file",
        extensions: &["eg"],
        media_types: &["text/x-earl-grey"],
        signatures: &[],
        related_formats: &[],
    },
};
