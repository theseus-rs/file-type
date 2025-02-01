use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858302: FileFormat = FileFormat {
    id: 105_858_302,
    puid: "wikidata/105858302",
    name: "E-Mail message (Var. 9)",
    extensions: &["eml"],
    media_types: &["message/rfc822"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x65, 0x74, 0x75, 0x72, 0x6E, 0x2D, 0x52, 0x65, 0x63, 0x65, 0x69, 0x70,
                    0x74, 0x2D, 0x54, 0x6F, 0x3A, 0x20, 0x3C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
