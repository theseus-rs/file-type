use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856792: FileFormat = FileFormat {
    id: 105_856_792,
    source_type: SourceType::Wikidata,
    name: "Applause Graph/Chart",
    extensions: &["g"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x68, 0x61, 0x72, 0x74, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x31, 0x2E,
                    0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
