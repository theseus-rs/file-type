use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1654: FileFormat = FileFormat {
    id: 2_481,
    puid: "fmt/1654",
    name: "Palm Database ImageViewer Format",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(60),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x76, 0x49, 0x4D, 0x47, 0x56, 0x69, 0x65, 0x77,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
