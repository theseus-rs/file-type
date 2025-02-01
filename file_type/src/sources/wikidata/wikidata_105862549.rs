use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862549: FileFormat = FileFormat {
    id: 105_862_549,
    puid: "wikidata/105862549",
    name: "Package Download Profile",
    extensions: &["mlf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x20, 0x44, 0x6F, 0x77, 0x6E,
                    0x6C, 0x6F, 0x61, 0x64, 0x20, 0x50, 0x72, 0x6F, 0x66, 0x69, 0x6C, 0x65, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
