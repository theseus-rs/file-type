use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861622: FileFormat = FileFormat {
    id: 105_861_622,
    source_type: SourceType::Wikidata,
    name: "LaserDRW drawing",
    extensions: &["lyz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x4C, 0x59, 0x5A])],
            },
        }],
    }],
    related_formats: &[],
};
