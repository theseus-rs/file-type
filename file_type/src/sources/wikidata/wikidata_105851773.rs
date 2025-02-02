use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851773: FileFormat = FileFormat {
    id: 105_851_773,
    source_type: SourceType::Wikidata,
    name: "MEKA savestate",
    extensions: &["s00"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x45, 0x4B, 0x41, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
