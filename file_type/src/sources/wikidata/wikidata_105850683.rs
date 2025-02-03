use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850683: FileFormat = FileFormat {
    id: 105_850_683,
    source_type: SourceType::Wikidata,
    name: "Battery 3 Drum Kit",
    extensions: &["kt3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x12, 0x90, 0xA8, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x72, 0x2A, 0x01,
                    0x3E, 0x01, 0x00, 0x08, 0x00, 0x02, 0x02, 0x33, 0x74, 0x61, 0x42,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
