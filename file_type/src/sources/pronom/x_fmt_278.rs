use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_278: FileFormat = FileFormat {
    id: 424,
    puid: "x-fmt/278",
    name: "RealAudio",
    extensions: &["ra"],
    media_types: &["audio/vnd.rn-realaudio"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x72, 0x61, 0xFD, 0x00, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
