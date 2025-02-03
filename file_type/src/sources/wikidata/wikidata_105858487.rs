use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858487: FileFormat = FileFormat {
    id: 105_858_487,
    source_type: SourceType::Wikidata,
    name: "bigWig Track Format",
    extensions: &["bigwig", "bw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x26, 0xFC, 0x8F, 0x88, 0x04, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x26, 0xFC, 0x8F, 0x88, 0x04, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
