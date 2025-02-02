use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851418: FileFormat = FileFormat {
    id: 105_851_418,
    source_type: SourceType::Wikidata,
    name: "Terragen terrain",
    extensions: &["ter", "terrain"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x45, 0x52, 0x52, 0x41, 0x47, 0x45, 0x4E, 0x54, 0x45, 0x52, 0x52,
                        0x41, 0x49, 0x4E, 0x20,
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
                        0x54, 0x45, 0x52, 0x52, 0x41, 0x47, 0x45, 0x4E, 0x54, 0x45, 0x52, 0x52,
                        0x41, 0x49, 0x4E, 0x20,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
