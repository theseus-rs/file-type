use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850915: FileFormat = FileFormat {
    id: 105_850_915,
    source_type: SourceType::Wikidata,
    name: "Text - SCSU encoded",
    extensions: &["txt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0E, 0xFE, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
