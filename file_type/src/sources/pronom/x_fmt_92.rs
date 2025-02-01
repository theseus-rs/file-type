use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_92: FileFormat = FileFormat {
    id: 139,
    puid: "x-fmt/92",
    name: "Adobe Photoshop",
    extensions: &["psd", "pdd"],
    media_types: &["image/vnd.adobe.photoshop"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x38, 0x42, 0x50, 0x53, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_801,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
