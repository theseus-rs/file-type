use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856968: FileFormat = FileFormat {
    id: 105_856_968,
    source_type: SourceType::Wikidata,
    name: "Mind Games - Renju saved game",
    extensions: &["gam"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x65, 0x6E, 0x6A, 0x75, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
