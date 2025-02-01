use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853523: FileFormat = FileFormat {
    id: 105_853_523,
    puid: "wikidata/105853523",
    name: "Zero-X Color scheme",
    extensions: &["zco"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x5A, 0x65, 0x72, 0x6F, 0x2D, 0x58, 0x20, 0x43, 0x6F, 0x6C, 0x6F, 0x72,
                    0x20, 0x53, 0x63, 0x68, 0x65, 0x6D, 0x65, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
