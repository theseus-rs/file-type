use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864790: FileFormat = FileFormat {
    id: 105_864_790,
    source_type: SourceType::Wikidata,
    name: "Particles format (big-endian)",
    extensions: &["pb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0xFF, 0xFF, 0x98])],
            },
        }],
    }],
    related_formats: &[],
};
