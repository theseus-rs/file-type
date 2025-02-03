use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856918: FileFormat = FileFormat {
    id: 105_856_918,
    source_type: SourceType::Wikidata,
    name: "DeluxePaint Gallery",
    extensions: &["gal"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x37, 0x12, 0x00, 0x00, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
