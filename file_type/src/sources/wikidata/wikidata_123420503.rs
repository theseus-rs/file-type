use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123420503: FileFormat = FileFormat {
    id: 123_420_503,
    source_type: SourceType::Wikidata,
    name: "DropBox file",
    extensions: &["dbox"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
