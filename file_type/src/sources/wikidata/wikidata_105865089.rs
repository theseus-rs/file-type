use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865089: FileFormat = FileFormat {
    id: 105_865_089,
    source_type: SourceType::Wikidata,
    name: "PathMinder Applications Menus (v4.00)",
    extensions: &["pm4"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4D, 0x34, 0x2E, 0x30, 0x30, 0x20, 0x41, 0x70, 0x70, 0x6C, 0x69, 0x63,
                    0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x4D, 0x65, 0x6E, 0x75, 0x73, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
