use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851532: FileFormat = FileFormat {
    id: 105_851_532,
    source_type: SourceType::Wikidata,
    name: "Ports of Call savegame (v2.0/PC)",
    extensions: &["trp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x44, 0x4B, 0x20, 0x20, 0x50, 0x4F, 0x43, 0x20, 0x20, 0x32, 0x2E, 0x30,
                    0x20, 0x28, 0x43, 0x29, 0x20, 0x31, 0x39, 0x39, 0x34, 0x2C, 0x20, 0x4D, 0x75,
                    0x65, 0x6E, 0x63, 0x68, 0x65, 0x6E, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
