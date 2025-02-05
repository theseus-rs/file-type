use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2830: FileFormat = FileFormat {
    id: 2_830,
    source_type: SourceType::Pronom,
    name: "Papyrus Document",
    extensions: &["pap", "pav", "pbf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x41, 0x50, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
