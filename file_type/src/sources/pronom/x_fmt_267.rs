use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_267: FileFormat = FileFormat {
    id: 387,
    puid: "x-fmt/267",
    name: "BZIP Compressed Archive",
    extensions: &["bz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x5A, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
