use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855483: FileFormat = FileFormat {
    id: 105_855_483,
    source_type: SourceType::Wikidata,
    name: "Fuchs Tracker module",
    extensions: &["ft", "fuchs"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4F, 0x4E, 0x47])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4F, 0x4E, 0x47])],
                },
            }],
        },
    ],
    related_formats: &[],
};
