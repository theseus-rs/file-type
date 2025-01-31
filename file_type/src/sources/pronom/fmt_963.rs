use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_963: FileFormat = FileFormat {
    id: 1_768,
    puid: "fmt/963",
    name: "OMNIC Spectral Data File",
    extensions: &["spa"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x70, 0x65, 0x63, 0x74, 0x72, 0x61, 0x6C, 0x20, 0x44, 0x61, 0x74, 0x61,
                    0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
