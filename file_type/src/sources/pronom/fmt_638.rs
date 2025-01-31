use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_638: FileFormat = FileFormat {
    id: 1_438,
    puid: "fmt/638",
    name: "SPSS Data File",
    extensions: &["sav"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x46, 0x4C, 0x32, 0x40, 0x28, 0x23, 0x29,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_723,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
