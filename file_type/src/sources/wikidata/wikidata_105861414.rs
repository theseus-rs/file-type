use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861414: FileFormat = FileFormat {
    id: 105_861_414,
    source_type: SourceType::Wikidata,
    name: "Frogans Short-cut",
    extensions: &["ltf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x4C, 0x45, 0x41, 0x50, 0x54, 0x4F, 0x46, 0x52, 0x4F, 0x47, 0x41, 0x4E,
                    0x53, 0x20, 0x41, 0x44, 0x44, 0x52, 0x45, 0x53, 0x53, 0x3D, 0x22, 0x66, 0x72,
                    0x6F, 0x67, 0x61, 0x6E, 0x73, 0x2A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
