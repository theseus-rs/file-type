use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858564: FileFormat = FileFormat {
    id: 105_858_564,
    source_type: SourceType::Wikidata,
    name: "GTA: San Andreas save game (v2.00 PC German)",
    extensions: &["b"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x22, 0xCC, 0x31, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
