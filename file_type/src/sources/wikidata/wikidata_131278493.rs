use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131278493: FileFormat = FileFormat {
    id: 131_278_493,
    source_type: SourceType::Wikidata,
    name: "Test Anything Protocol output file",
    extensions: &["tap"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
