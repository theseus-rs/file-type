use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858541: FileFormat = FileFormat {
    id: 105_858_541,
    source_type: SourceType::Wikidata,
    name: "BassBox speaker design (v6)",
    extensions: &["bb6"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x36, 0x2E, 0x30, 0x30, 0x30, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
