use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858214: FileFormat = FileFormat {
    id: 105_858_214,
    source_type: SourceType::Wikidata,
    name: "EzCad Drawing",
    extensions: &["ezd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x00, 0x5A, 0x00, 0x43, 0x00, 0x41, 0x00, 0x44, 0x00, 0x55, 0x00, 0x4E,
                    0x00, 0x49, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
