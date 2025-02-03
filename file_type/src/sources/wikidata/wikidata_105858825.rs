use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858825: FileFormat = FileFormat {
    id: 105_858_825,
    source_type: SourceType::Wikidata,
    name: "GTA: San Andreas save game (v1 PS2)",
    extensions: &["b"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0xDC, 0x1D, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
