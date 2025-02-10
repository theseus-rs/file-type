use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_55239129: FileType = FileType {
    file_format: &FileFormat {
        id: 55_239_129,
        source_type: SourceType::Wikidata,
        name: "CBOR Web Token format",
        extensions: &["cwt"],
        media_types: &["application/cwt"],
        signatures: &[],
        related_formats: &[],
    },
};
