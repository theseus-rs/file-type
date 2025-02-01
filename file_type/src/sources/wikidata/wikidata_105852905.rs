use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852905: FileFormat = FileFormat {
    id: 105_852_905,
    puid: "wikidata/105852905",
    name: "Sangduck Sprite",
    extensions: &["spr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x61, 0x6E, 0x67, 0x64, 0x75, 0x63, 0x6B, 0x20, 0x53, 0x70, 0x72, 0x69,
                    0x74, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
