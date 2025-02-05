use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128693897: FileFormat = FileFormat {
    id: 128_693_897,
    source_type: SourceType::Wikidata,
    name: "boo script",
    extensions: &["boo"],
    media_types: &["text/x-boo"],
    signatures: &[],
    related_formats: &[],
};
