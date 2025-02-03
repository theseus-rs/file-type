use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855501: FileFormat = FileFormat {
    id: 105_855_501,
    source_type: SourceType::Wikidata,
    name: "Micro Magic Forms in Flight Objects",
    extensions: &["mmo"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6D, 0x69, 0x63, 0x72, 0x6F, 0x20, 0x6D, 0x61, 0x67, 0x69, 0x63, 0x20, 0x6F,
                    0x62, 0x6A, 0x65, 0x63, 0x74, 0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
