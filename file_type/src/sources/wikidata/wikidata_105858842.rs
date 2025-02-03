use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858842: FileFormat = FileFormat {
    id: 105_858_842,
    source_type: SourceType::Wikidata,
    name: "VIV/BIGF Electronic Arts Game Archive",
    extensions: &["big", "viv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x49, 0x47, 0x46])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x49, 0x47, 0x46])],
                },
            }],
        },
    ],
    related_formats: &[],
};
