use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_531: FileFormat = FileFormat {
    id: 1_318,
    puid: "fmt/531",
    name: "AutoCAD Drawing",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x31, 0x30, 0x32, 0x37])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_221,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
