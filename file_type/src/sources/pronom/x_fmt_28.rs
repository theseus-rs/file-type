use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_28: FileFormat = FileFormat {
    id: 59,
    puid: "x-fmt/28",
    name: "CALS Compressed Bitmap",
    extensions: &["cal"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x73, 0x72, 0x63, 0x64, 0x6F, 0x63, 0x69, 0x64, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
