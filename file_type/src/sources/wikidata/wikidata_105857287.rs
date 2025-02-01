use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857287: FileFormat = FileFormat {
    id: 105_857_287,
    puid: "wikidata/105857287",
    name: "Home Accounts account(v2)",
    extensions: &["ha2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x41, 0x32, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
