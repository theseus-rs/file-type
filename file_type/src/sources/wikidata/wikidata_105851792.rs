use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851792: FileFormat = FileFormat {
    id: 105_851_792,
    source_type: SourceType::Wikidata,
    name: "StockChartX data",
    extensions: &["stx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x74, 0x6F, 0x63, 0x6B, 0x43, 0x68, 0x61, 0x72, 0x74, 0x58,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
