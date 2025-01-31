use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859179: FileFormat = FileFormat {
    id: 105_859_179,
    puid: "wikidata/105859179",
    name: "TomTom info (generic)",
    extensions: &["bif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x54, 0x6F, 0x6D, 0x54, 0x6F, 0x6D])],
            },
        }],
    }],
    related_formats: &[],
};
