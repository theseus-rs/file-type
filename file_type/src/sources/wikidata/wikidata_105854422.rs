use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854422: FileFormat = FileFormat {
    id: 105_854_422,
    puid: "wikidata/105854422",
    name: "Antenna project",
    extensions: &["ata"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x51, 0x75, 0x69, 0x63, 0x6B, 0x20, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65,
                    0x20, 0xA9, 0x32, 0x30, 0x30, 0x33, 0x20, 0x4A, 0x75, 0x6C, 0x69, 0x61, 0x6E,
                    0x20, 0x53, 0x70, 0x65, 0x6E, 0x63, 0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
