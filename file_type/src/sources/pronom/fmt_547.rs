use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_547: FileFormat = FileFormat {
    id: 1_335,
    puid: "fmt/547",
    name: "Macromedia FreeHand",
    extensions: &["fh10"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x1C, 0x01, 0x00, 0x00, 0x02, 0x00, 0x04, 0x1C, 0x01, 0x14, 0x00, 0x02, 0x00,
                    0x14, 0x1C, 0x01, 0x16, 0x00, 0x02, 0x00, 0x08, 0x1C, 0x01, 0x1E, 0x00, 0x0A,
                    0x46, 0x72, 0x65, 0x65, 0x48, 0x61, 0x6E, 0x64, 0x31, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
