use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864464: FileFormat = FileFormat {
    id: 105_864_464,
    puid: "wikidata/105864464",
    name: "Applause Palette",
    extensions: &["pal"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x61, 0x6C, 0x65, 0x74, 0x74, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20,
                    0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
