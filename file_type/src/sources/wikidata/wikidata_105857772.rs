use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857772: FileFormat = FileFormat {
    id: 105_857_772,
    source_type: SourceType::Wikidata,
    name: "iMELODY sound/music (header less)",
    extensions: &["imelody", "imy"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x45, 0x4C, 0x4F, 0x44, 0x59, 0x3A])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x45, 0x4C, 0x4F, 0x44, 0x59, 0x3A])],
                },
            }],
        },
    ],
    related_formats: &[],
};
