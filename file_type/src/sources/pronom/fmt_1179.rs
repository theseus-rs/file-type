use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1179: FileFormat = FileFormat {
    id: 1_989,
    puid: "fmt/1179",
    name: "Away3D Data Format",
    extensions: &["awd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x57, 0x44, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
