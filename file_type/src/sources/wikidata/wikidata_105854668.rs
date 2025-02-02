use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854668: FileFormat = FileFormat {
    id: 105_854_668,
    source_type: SourceType::Wikidata,
    name: "DJarc compressed archive",
    extensions: &["dja"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x4A, 0x61, 0x72, 0x63])],
            },
        }],
    }],
    related_formats: &[],
};
