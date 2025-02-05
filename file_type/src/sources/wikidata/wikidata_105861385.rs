use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861385: FileFormat = FileFormat {
    id: 105_861_385,
    source_type: SourceType::Wikidata,
    name: "PolyPlot Layer (v3.0)",
    extensions: &["lay"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x19, 0x08, 0x50, 0x6F, 0x6C, 0x79, 0x70, 0x6C, 0x6F, 0x74, 0x20, 0x33, 0x2E,
                    0x30, 0x20, 0x4C, 0x61, 0x79, 0x65, 0x72, 0x64, 0x61, 0x74, 0x65, 0x69, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
