use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855526: FileFormat = FileFormat {
    id: 105_855_526,
    source_type: SourceType::Wikidata,
    name: "6502 binary relocation format (v1)",
    extensions: &["o65"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x00, 0x6F, 0x36, 0x35, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
