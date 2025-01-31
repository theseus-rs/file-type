use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1359: FileFormat = FileFormat {
    id: 2_177,
    puid: "fmt/1359",
    name: "Softdisk Text Compressor",
    extensions: &["ctx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x03, 0x43, 0x54, 0x30, 0x30, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
