use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861366: FileFormat = FileFormat {
    id: 105_861_366,
    source_type: SourceType::Wikidata,
    name: "MicroLathe object",
    extensions: &["lat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xE6, 0x4C, 0x41, 0x54, 0x48, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
