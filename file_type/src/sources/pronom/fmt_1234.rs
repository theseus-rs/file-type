use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1234: FileFormat = FileFormat {
    id: 2_052,
    puid: "fmt/1234",
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
