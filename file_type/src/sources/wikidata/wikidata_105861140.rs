use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861140: FileFormat = FileFormat {
    id: 105_861_140,
    source_type: SourceType::Wikidata,
    name: "Microtime Animal level data",
    extensions: &["lvl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x57, 0x2A, 0x33, 0x44, 0x72, 0x2A, 0x56, 0x65, 0x72, 0x2A, 0x30, 0x37,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
