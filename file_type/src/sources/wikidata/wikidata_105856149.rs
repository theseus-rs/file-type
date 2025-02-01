use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856149: FileFormat = FileFormat {
    id: 105_856_149,
    puid: "wikidata/105856149",
    name: "GRAFIT layout",
    extensions: &["des"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x47, 0x52,
                    0x41, 0x46, 0x49, 0x54, 0x20, 0x6C, 0x61, 0x79, 0x6F, 0x75, 0x74, 0x20, 0x66,
                    0x69, 0x6C, 0x65, 0x2E, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
