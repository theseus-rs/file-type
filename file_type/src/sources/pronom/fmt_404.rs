use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_404: FileFormat = FileFormat {
    id: 1_152,
    puid: "fmt/404",
    name: "RealAudio",
    extensions: &["ra"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2E, 0x72, 0x61, 0xFD, 0x00, 0x04, 0x00, 0x00, 0x2E, 0x72, 0x61, 0x34,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
