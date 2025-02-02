use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856880: FileFormat = FileFormat {
    id: 105_856_880,
    source_type: SourceType::Wikidata,
    name: "Graphtec vector data / drawing",
    extensions: &["gsd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x52, 0x41, 0x50, 0x48, 0x54, 0x45, 0x43, 0x20, 0x50, 0x52, 0x54, 0x26,
                    0x43, 0x55, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
