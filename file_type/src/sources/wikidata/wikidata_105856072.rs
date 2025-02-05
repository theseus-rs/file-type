use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856072: FileFormat = FileFormat {
    id: 105_856_072,
    source_type: SourceType::Wikidata,
    name: "PolyPlot Diagram definition",
    extensions: &["dia"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2C, 0x00, 0x24, 0x08, 0x08, 0x08, 0x50, 0x6F, 0x6C, 0x79, 0x70, 0x6C, 0x6F,
                    0x74, 0x20, 0x44, 0x69, 0x61, 0x67, 0x72, 0x61, 0x6D, 0x6D, 0x64, 0x65, 0x66,
                    0x69, 0x6E, 0x74, 0x69, 0x6F, 0x6E, 0x73, 0x64, 0x61, 0x74, 0x65, 0x69, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
