use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859519: FileFormat = FileFormat {
    id: 105_859_519,
    source_type: SourceType::Wikidata,
    name: "VirtualDJ audio Sample",
    extensions: &["vdjsample"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x44, 0x4A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
