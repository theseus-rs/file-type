use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854321: FileFormat = FileFormat {
    id: 105_854_321,
    source_type: SourceType::Wikidata,
    name: "Atari V4E GFA Basic v3.6 executable",
    extensions: &["app"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x60, 0x1A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
