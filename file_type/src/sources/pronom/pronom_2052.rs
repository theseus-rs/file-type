use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2052: FileFormat = FileFormat {
    id: 2_052,
    source_type: SourceType::Pronom,
    name: "Smacker Video",
    extensions: &["smk"],
    media_types: &["video/vnd.radgamettools.smacker"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x53, 0x4D, 0x4B, 0x32]),
                    Token::WildcardCount(96),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
