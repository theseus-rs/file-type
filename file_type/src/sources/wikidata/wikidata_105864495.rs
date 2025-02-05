use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864495: FileFormat = FileFormat {
    id: 105_864_495,
    source_type: SourceType::Wikidata,
    name: "Particles format (little-endian)",
    extensions: &["pb"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x98, 0xFF, 0xFF, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
