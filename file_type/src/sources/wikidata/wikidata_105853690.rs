use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853690: FileFormat = FileFormat {
    id: 105_853_690,
    source_type: SourceType::Wikidata,
    name: "AcuCorp AcuCOBOL license",
    extensions: &["alc", "vlc"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x20, 0x41, 0x43, 0x55, 0x43, 0x4F, 0x42, 0x4F, 0x4C, 0x20,
                        0x6C, 0x69, 0x63, 0x65, 0x6E, 0x73, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x65,
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
                        0x23, 0x20, 0x20, 0x41, 0x43, 0x55, 0x43, 0x4F, 0x42, 0x4F, 0x4C, 0x20,
                        0x6C, 0x69, 0x63, 0x65, 0x6E, 0x73, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
