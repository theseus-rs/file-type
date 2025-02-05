use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1778: FileFormat = FileFormat {
    id: 1_778,
    source_type: SourceType::Pronom,
    name: "DTS Coherent Acoustics (DCA) Audio",
    extensions: &["dts"],
    media_types: &["audio/vnd.dts"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7F, 0xFE, 0x80, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
