use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1897: FileFormat = FileFormat {
    id: 2_753,
    puid: "fmt/1897",
    name: "Ptex File Format",
    extensions: &["ptx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x74, 0x65, 0x78])],
            },
        }],
    }],
    related_formats: &[],
};
