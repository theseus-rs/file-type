use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851030: FileFormat = FileFormat {
    id: 105_851_030,
    source_type: SourceType::Wikidata,
    name: "TK Solver module (v1)",
    extensions: &["tk"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x4B, 0x2B, 0x31, 0x0D, 0x0A, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
