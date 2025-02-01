use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850268: FileFormat = FileFormat {
    id: 105_850_268,
    puid: "wikidata/105850268",
    name: "Compact compressed data (alt)",
    extensions: &["c"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1F, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
