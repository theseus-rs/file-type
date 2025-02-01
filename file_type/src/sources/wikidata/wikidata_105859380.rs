use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859380: FileFormat = FileFormat {
    id: 105_859_380,
    puid: "wikidata/105859380",
    name: "QuickReport Report",
    extensions: &["qrp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0x0A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
