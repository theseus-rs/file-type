use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855933: FileFormat = FileFormat {
    id: 105_855_933,
    puid: "wikidata/105855933",
    name: "DateBook Archive",
    extensions: &["dba"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x01, 0x42, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
