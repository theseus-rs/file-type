use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1952708: FileType = FileType {
    file_format: &FileFormat {
        id: 1_952_708,
        source_type: SourceType::Wikidata,
        name: "FILE_ID.DIZ",
        extensions: &["diz"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
