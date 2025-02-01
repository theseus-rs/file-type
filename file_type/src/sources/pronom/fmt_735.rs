use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_735: FileFormat = FileFormat {
    id: 1_534,
    puid: "fmt/735",
    name: "Dolby Digital AC-3",
    extensions: &["ac3"],
    media_types: &["audio/ac3"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0B, 0x77])],
            },
        }],
    }],
    related_formats: &[],
};
