use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853528: FileFormat = FileFormat {
    id: 105_853_528,
    source_type: SourceType::Wikidata,
    name: "ZAP Patch",
    extensions: &["zap"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x34, 0x32, 0x36, 0x20, 0x30, 0x37, 0x31, 0x20, 0x30, 0x34, 0x37, 0x20, 0x35,
                    0x30, 0x31, 0x20, 0x31, 0x37, 0x34, 0x20, 0x33, 0x32, 0x31, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
