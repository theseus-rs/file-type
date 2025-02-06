use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1858: FileFormat = FileFormat {
    id: 1_858,
    source_type: SourceType::Pronom,
    name: "Adobe Audio Waveform",
    extensions: &["pek"],
    media_types: &[],
    signatures: &[Signature {
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
