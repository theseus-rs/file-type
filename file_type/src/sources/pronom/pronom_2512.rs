use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2512: FileFormat = FileFormat {
    id: 2_512,
    source_type: SourceType::Pronom,
    name: "Covox ADPCM Audio Files",
    extensions: &["v8", "cvx", "v2s", "v3s", "v4s", "vmf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0x55, 0xFF, 0xAA, 0xFF, 0x55, 0xFF, 0xAA,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
