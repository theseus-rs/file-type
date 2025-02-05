use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858502: FileFormat = FileFormat {
    id: 105_858_502,
    source_type: SourceType::Wikidata,
    name: "QuickBMS script (with XML header)",
    extensions: &["bms"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x62, 0x6D, 0x73, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
