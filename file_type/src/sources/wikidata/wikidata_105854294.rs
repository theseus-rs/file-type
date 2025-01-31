use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854294: FileFormat = FileFormat {
    id: 105_854_294,
    puid: "wikidata/105854294",
    name: "AZZ Cardfile data (Text)",
    extensions: &["tmp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x59, 0x31, 0x44, 0x4C, 0x20, 0x41, 0x6E, 0x6F, 0x74, 0x68, 0x65, 0x72,
                    0x20, 0x63, 0x61, 0x72, 0x64, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x2D, 0x20, 0x54,
                    0x58, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
