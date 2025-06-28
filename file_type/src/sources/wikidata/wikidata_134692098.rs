use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134692098: FileType = FileType {
    file_format: &FileFormat {
        id: 134_692_098,
        source_type: SourceType::Wikidata,
        name: "NooJ syntactic grammar file",
        extensions: &["nog"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
