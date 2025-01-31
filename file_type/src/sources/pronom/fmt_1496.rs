use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1496: FileFormat = FileFormat {
    id: 2_319,
    puid: "fmt/1496",
    name: "ZoomBrowser Ex Thumbnail Cache",
    extensions: &["info"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7A, 0x62, 0x65, 0x78])],
            },
        }],
    }],
    related_formats: &[],
};
