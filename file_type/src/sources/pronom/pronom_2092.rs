use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2092: FileFormat = FileFormat {
    id: 2_092,
    source_type: SourceType::Pronom,
    name: "Sonic Scenarist Closed Caption Format",
    extensions: &["scc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x63, 0x65, 0x6E, 0x61, 0x72, 0x69, 0x73, 0x74, 0x5F, 0x53, 0x43, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
