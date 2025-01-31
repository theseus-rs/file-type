use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856708: FileFormat = FileFormat {
    id: 105_856_708,
    puid: "wikidata/105856708",
    name: "UAE Saved State",
    extensions: &["uss"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x53, 0x46, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
