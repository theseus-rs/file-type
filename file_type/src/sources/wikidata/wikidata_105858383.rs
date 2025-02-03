use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858383: FileFormat = FileFormat {
    id: 105_858_383,
    source_type: SourceType::Wikidata,
    name: "E-Mail message (Var. 8)",
    extensions: &["eml"],
    media_types: &["message/rfc822"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2D, 0x49, 0x44, 0x3A, 0x20, 0x3C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
