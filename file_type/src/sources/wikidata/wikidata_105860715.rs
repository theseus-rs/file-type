use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860715: FileFormat = FileFormat {
    id: 105_860_715,
    source_type: SourceType::Wikidata,
    name: "Looking Glass Resource data",
    extensions: &["res"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x47, 0x20, 0x52, 0x65, 0x73, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x76,
                    0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
