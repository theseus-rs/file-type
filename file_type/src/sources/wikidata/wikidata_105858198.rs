use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858198: FileFormat = FileFormat {
    id: 105_858_198,
    puid: "wikidata/105858198",
    name: "E-Mail message (Var. 10)",
    extensions: &["eml"],
    media_types: &["message/rfc822"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x78, 0x2D, 0x73, 0x74, 0x6F, 0x72, 0x65, 0x2D, 0x69, 0x6E, 0x66, 0x6F, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
