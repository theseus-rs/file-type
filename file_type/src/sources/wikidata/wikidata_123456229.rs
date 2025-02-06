use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123456229: FileFormat = FileFormat {
    id: 123_456_229,
    source_type: SourceType::Wikidata,
    name: "CFW Form file",
    extensions: &["cfw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
