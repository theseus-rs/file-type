use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1554: FileFormat = FileFormat {
    id: 2_379,
    puid: "fmt/1554",
    name: "DNA Sequence Chromatogram File",
    extensions: &["scf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x73, 0x63, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
