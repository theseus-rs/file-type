use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852170: FileFormat = FileFormat {
    id: 105_852_170,
    source_type: SourceType::Wikidata,
    name: "SubStation Alpha Subtitle",
    extensions: &["ass", "ssa"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x20, 0x49, 0x6E, 0x66, 0x6F,
                        0x5D,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x20, 0x49, 0x6E, 0x66, 0x6F,
                        0x5D,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
