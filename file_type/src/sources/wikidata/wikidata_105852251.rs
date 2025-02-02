use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852251: FileFormat = FileFormat {
    id: 105_852_251,
    source_type: SourceType::Wikidata,
    name: "Scid player/event/site/round data",
    extensions: &["sn3", "sn4"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x63, 0x69, 0x64, 0x2E, 0x73, 0x6E, 0x00,
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
                        0x53, 0x63, 0x69, 0x64, 0x2E, 0x73, 0x6E, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
