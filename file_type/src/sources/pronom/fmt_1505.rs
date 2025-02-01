use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1505: FileFormat = FileFormat {
    id: 2_328,
    puid: "fmt/1505",
    name: "Agisoft Point Cloud",
    extensions: &["oc3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x43, 0x01, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
