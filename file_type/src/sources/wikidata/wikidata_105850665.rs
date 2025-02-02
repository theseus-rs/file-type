use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850665: FileFormat = FileFormat {
    id: 105_850_665,
    source_type: SourceType::Wikidata,
    name: "KVIrc Theme",
    extensions: &["kvt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x56, 0x50, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
