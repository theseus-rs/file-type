use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858255: FileFormat = FileFormat {
    id: 105_858_255,
    puid: "wikidata/105858255",
    name: "E-Mail message (Var. 3)",
    extensions: &["eml"],
    media_types: &["message/rfc822"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
