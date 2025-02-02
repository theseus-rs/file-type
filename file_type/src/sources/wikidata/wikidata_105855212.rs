use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855212: FileFormat = FileFormat {
    id: 105_855_212,
    source_type: SourceType::Wikidata,
    name: "Stunt Island Film",
    extensions: &["flm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x49, 0x4C, 0x4D, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
