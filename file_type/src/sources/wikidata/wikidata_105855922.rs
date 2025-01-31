use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855922: FileFormat = FileFormat {
    id: 105_855_922,
    puid: "wikidata/105855922",
    name: "Adventure Game Toolkit temporary strings",
    extensions: &["d$$"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2E, 0x20, 0x3E, 0x3E, 0x20, 0x54, 0x65, 0x6D, 0x70, 0x6F, 0x72, 0x61, 0x72,
                    0x79, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x6F,
                    0x66, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67, 0x73, 0x20, 0x66, 0x6F, 0x72,
                    0x20, 0x41, 0x47, 0x54, 0x20, 0x3C, 0x3C, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
