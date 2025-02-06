use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2749: FileFormat = FileFormat {
    id: 2_749,
    source_type: SourceType::Pronom,
    name: "Microsoft Agent File",
    extensions: &["acs"],
    media_types: &[],
    signatures: &[Signature {
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
