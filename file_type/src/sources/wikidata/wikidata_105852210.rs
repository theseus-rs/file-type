use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852210: FileFormat = FileFormat {
    id: 105_852_210,
    puid: "wikidata/105852210",
    name: "Accent DemoMaker Sequence",
    extensions: &["seq"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xA9, 0xD1, 0x6A, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
