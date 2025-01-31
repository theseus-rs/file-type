use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_996: FileFormat = FileFormat {
    id: 1_801,
    puid: "fmt/996",
    name: "Adobe Photoshop Large Document Format",
    extensions: &["psb"],
    media_types: &["image/vnd.adobe.photoshop"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x38, 0x42, 0x50, 0x53, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 139,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
