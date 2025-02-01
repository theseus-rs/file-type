use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1612: FileFormat = FileFormat {
    id: 2_439,
    puid: "fmt/1612",
    name: "XBIN (eXtended BIN)",
    extensions: &["xb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x42, 0x49, 0x4E, 0x1A])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 934,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
