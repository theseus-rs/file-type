use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_126: FileFormat = FileFormat {
    id: 181,
    puid: "x-fmt/126",
    name: "Microsoft Excel Chart",
    extensions: &["xlc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x09, 0x04, 0x06, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x20, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 680,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
