use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1893: FileFormat = FileFormat {
    id: 2_749,
    puid: "fmt/1893",
    name: "Microsoft Agent File",
    extensions: &["acs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xC3, 0xAB, 0xCD, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
