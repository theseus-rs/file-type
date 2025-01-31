use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853152: FileFormat = FileFormat {
    id: 105_853_152,
    puid: "wikidata/105853152",
    name: "Yamaha EX5 voices format",
    extensions: &["s1v"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x59, 0x31, 0x32, 0x30, 0x30, 0x56, 0x43, 0x41, 0x20, 0x20, 0x56, 0x31,
                    0x2E, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
