use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851045: FileFormat = FileFormat {
    id: 105_851_045,
    source_type: SourceType::Wikidata,
    name: "PolyPlot Text Definition",
    extensions: &["txd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x20, 0x08, 0x08, 0x08, 0x50, 0x6F, 0x6C, 0x79, 0x70, 0x6C, 0x6F, 0x74,
                    0x20, 0x54, 0x65, 0x78, 0x74, 0x64, 0x65, 0x66, 0x69, 0x6E, 0x74, 0x69, 0x6F,
                    0x6E, 0x73, 0x64, 0x61, 0x74, 0x65, 0x69, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
