use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_922: FileFormat = FileFormat {
    id: 1_727,
    puid: "fmt/922",
    name: "Xar Image Format",
    extensions: &["xar"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x58, 0x41, 0x52, 0x41, 0xA3, 0xA3, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
