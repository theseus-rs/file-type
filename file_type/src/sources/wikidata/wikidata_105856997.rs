use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856997: FileFormat = FileFormat {
    id: 105_856_997,
    puid: "wikidata/105856997",
    name: "MicroImages GPS Log (v1)",
    extensions: &["gps"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x69, 0x63, 0x72, 0x6F, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x73, 0x20, 0x47,
                    0x50, 0x53, 0x20, 0x4C, 0x6F, 0x67, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F,
                    0x6E, 0x20, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
