use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_424: FileFormat = FileFormat {
    id: 424,
    source_type: SourceType::Pronom,
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
