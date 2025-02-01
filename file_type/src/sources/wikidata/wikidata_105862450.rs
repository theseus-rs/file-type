use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862450: FileFormat = FileFormat {
    id: 105_862_450,
    puid: "wikidata/105862450",
    name: "MetaQuote / MetaTrader indicator",
    extensions: &["mq4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F, 0x2F, 0x2B, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
