use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859250: FileFormat = FileFormat {
    id: 105_859_250,
    puid: "wikidata/105859250",
    name: "GTA: San Andreas save game (v2.00 PC)",
    extensions: &["b"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF6, 0x8D, 0x14, 0xFD])],
            },
        }],
    }],
    related_formats: &[],
};
