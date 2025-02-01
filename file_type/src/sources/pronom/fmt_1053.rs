use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1053: FileFormat = FileFormat {
    id: 1_858,
    puid: "fmt/1053",
    name: "Adobe Audio Waveform",
    extensions: &["pek"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x11, 0x54, 0x23, 0x67])],
            },
        }],
    }],
    related_formats: &[],
};
