use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866455: FileFormat = FileFormat {
    id: 105_866_455,
    puid: "wikidata/105866455",
    name: "CUPL PLD Program format (with rem)",
    extensions: &["pld"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F, 0x2A])],
            },
        }],
    }],
    related_formats: &[],
};
