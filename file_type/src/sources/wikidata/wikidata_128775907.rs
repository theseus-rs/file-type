use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128775907: FileType = FileType {
    file_format: &FileFormat {
        id: 128_775_907,
        source_type: SourceType::Wikidata,
        name: "Coq file format",
        extensions: &["v"],
        media_types: &["text/x-coq"],
        signatures: &[],
        related_formats: &[],
    },
};
