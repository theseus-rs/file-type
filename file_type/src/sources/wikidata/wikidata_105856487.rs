use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856487: FileFormat = FileFormat {
    id: 105_856_487,
    source_type: SourceType::Wikidata,
    name: "Lotus 123 Worksheet (V2J)",
    extensions: &["wj3"],
    media_types: &["application/vnd.lotus-1-2-3"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x02, 0x00, 0x02, 0x06, 0x1F, 0x00, 0x02, 0x00, 0x10, 0x00, 0x06,
                    0x00, 0x08, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
