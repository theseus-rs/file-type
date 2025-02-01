use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860052: FileFormat = FileFormat {
    id: 105_860_052,
    puid: "wikidata/105860052",
    name: "Vividas video",
    extensions: &["viv"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x76, 0x69, 0x76, 0x69, 0x64, 0x61, 0x73])],
            },
        }],
    }],
    related_formats: &[],
};
