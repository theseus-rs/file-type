use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862952: FileFormat = FileFormat {
    id: 105_862_952,
    puid: "wikidata/105862952",
    name: "TROFF markup",
    extensions: &["me"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x5C, 0x22, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
