use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858107: FileFormat = FileFormat {
    id: 105_858_107,
    source_type: SourceType::Wikidata,
    name: "Infinity Engine exported player Character (v2.2)",
    extensions: &["chr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x48, 0x52, 0x20, 0x56, 0x32, 0x2E, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
