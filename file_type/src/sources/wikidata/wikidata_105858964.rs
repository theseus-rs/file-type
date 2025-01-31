use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858964: FileFormat = FileFormat {
    id: 105_858_964,
    puid: "wikidata/105858964",
    name: "Balsamiq Mockups prototype",
    extensions: &["bmml"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x6D, 0x6F, 0x63, 0x6B, 0x75, 0x70, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
                    0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x73, 0x6B, 0x69, 0x6E,
                    0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
