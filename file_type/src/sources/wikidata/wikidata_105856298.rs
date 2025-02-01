use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856298: FileFormat = FileFormat {
    id: 105_856_298,
    puid: "wikidata/105856298",
    name: "Runtime Software compressed disk image",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x1A, 0x52, 0x54, 0x53, 0x20, 0x43, 0x4F, 0x4D, 0x50, 0x52, 0x45, 0x53, 0x53,
                    0x45, 0x44, 0x20, 0x49, 0x4D, 0x41, 0x47, 0x45, 0x20, 0x56, 0x31, 0x2E, 0x30,
                    0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
