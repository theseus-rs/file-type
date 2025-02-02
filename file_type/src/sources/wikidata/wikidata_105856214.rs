use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856214: FileFormat = FileFormat {
    id: 105_856_214,
    source_type: SourceType::Wikidata,
    name: "Dynamically Linked Device Interface",
    extensions: &["dldi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xED, 0xA5, 0x8D, 0xBF, 0x20, 0x43, 0x68, 0x69, 0x73, 0x68, 0x6D, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
