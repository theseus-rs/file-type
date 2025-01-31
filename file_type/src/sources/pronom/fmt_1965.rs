use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1965: FileFormat = FileFormat {
    id: 2_830,
    puid: "fmt/1965",
    name: "Papyrus Document",
    extensions: &["pap", "pav", "pbf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x41, 0x50, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
