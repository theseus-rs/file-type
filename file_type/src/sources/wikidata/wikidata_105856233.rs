use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856233: FileFormat = FileFormat {
    id: 105_856_233,
    puid: "wikidata/105856233",
    name: "Dan Bricklin's Demo 2 demo",
    extensions: &["dbd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x79, 0x70, 0x65, 0x44, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
