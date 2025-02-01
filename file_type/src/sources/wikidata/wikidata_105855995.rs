use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855995: FileFormat = FileFormat {
    id: 105_855_995,
    puid: "wikidata/105855995",
    name: "IN:TOUCH telephone directory",
    extensions: &["dlr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x65, 0x6C, 0x65, 0x70, 0x68, 0x6F, 0x6E, 0x65, 0x20, 0x44, 0x69, 0x72,
                    0x65, 0x63, 0x74, 0x6F, 0x72, 0x79, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
