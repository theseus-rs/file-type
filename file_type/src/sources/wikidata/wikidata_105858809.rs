use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858809: FileFormat = FileFormat {
    id: 105_858_809,
    source_type: SourceType::Wikidata,
    name: "packPNM compressed Portable PixMap (binary) bitmap",
    extensions: &["ppn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x36])],
            },
        }],
    }],
    related_formats: &[],
};
