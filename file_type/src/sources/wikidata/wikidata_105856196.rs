use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856196: FileFormat = FileFormat {
    id: 105_856_196,
    source_type: SourceType::Wikidata,
    name: "Klasik picture Output Driver",
    extensions: &["dro"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x2E, 0x54, 0x2E, 0x20, 0x4B, 0x6C, 0x61, 0x73, 0x69, 0x6B, 0x20, 0x2D,
                    0x20, 0x70, 0x69, 0x63, 0x74, 0x75, 0x72, 0x65, 0x20, 0x64, 0x72, 0x69, 0x76,
                    0x65, 0x72, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
