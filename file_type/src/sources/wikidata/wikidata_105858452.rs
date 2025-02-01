use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858452: FileFormat = FileFormat {
    id: 105_858_452,
    puid: "wikidata/105858452",
    name: "E-Mail message (Var. 5)",
    extensions: &["eml"],
    media_types: &["message/rfc822"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
