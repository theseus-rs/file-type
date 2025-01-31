use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_304: FileFormat = FileFormat {
    id: 458,
    puid: "x-fmt/304",
    name: "Aldus Freehand Drawing",
    extensions: &["fh4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x41, 0x47, 0x44, 0x31]),
                    Token::WildcardCountRange(64, 1_024),
                    Token::Literal(&[
                        0x41, 0x6C, 0x64, 0x75, 0x73, 0x20, 0x46, 0x72, 0x65, 0x65, 0x48, 0x61,
                        0x6E, 0x64, 0x20, 0x34,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 87,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
