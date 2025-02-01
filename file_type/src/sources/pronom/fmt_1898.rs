use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1898: FileFormat = FileFormat {
    id: 2_754,
    puid: "fmt/1898",
    name: "Perfect ZX Tape (PZX) Image Format",
    extensions: &["pzx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x5A, 0x58, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
