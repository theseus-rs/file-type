use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2470: FileFormat = FileFormat {
    id: 2_470,
    source_type: SourceType::Pronom,
    name: "Lenel Network Video Recorder File",
    extensions: &["lnr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x4E, 0x52, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
