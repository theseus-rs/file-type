use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849714: FileFormat = FileFormat {
    id: 105_849_714,
    source_type: SourceType::Wikidata,
    name: "CircuitMaker schematic",
    extensions: &["ckt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x4D, 0x61, 0x6B, 0x65, 0x72, 0x20,
                    0x54, 0x65, 0x78, 0x74, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
