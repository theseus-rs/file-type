use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1274: FileFormat = FileFormat {
    id: 2_092,
    puid: "fmt/1274",
    name: "Sonic Scenarist Closed Caption Format",
    extensions: &["scc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
