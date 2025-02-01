use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864947: FileFormat = FileFormat {
    id: 105_864_947,
    puid: "wikidata/105864947",
    name: "CUPL PLD Program format",
    extensions: &["pld"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x61, 0x6D, 0x65])],
            },
        }],
    }],
    related_formats: &[],
};
