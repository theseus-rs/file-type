use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858825: FileFormat = FileFormat {
    id: 105_858_825,
    puid: "wikidata/105858825",
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
