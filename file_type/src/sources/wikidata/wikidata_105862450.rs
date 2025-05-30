use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862450: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_450,
        source_type: SourceType::Wikidata,
        name: "MetaQuote / MetaTrader indicator",
        extensions: &["mq4"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x2F, 0x2B, 0x2D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
