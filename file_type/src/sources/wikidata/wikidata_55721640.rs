use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55721640: FileFormat = FileFormat {
    id: 55_721_640,
    puid: "wikidata/55721640",
    name: "AmiraMesh ASCII file format",
    extensions: &["am", "amiramesh", "hx"],
    media_types: &["text/plain", "text/plain", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x41, 0x6D, 0x69, 0x72, 0x61, 0x4D, 0x65, 0x73, 0x68, 0x20,
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
                        0x23, 0x20, 0x41, 0x6D, 0x69, 0x72, 0x61, 0x4D, 0x65, 0x73, 0x68, 0x20,
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
                        0x23, 0x20, 0x41, 0x6D, 0x69, 0x72, 0x61, 0x4D, 0x65, 0x73, 0x68, 0x20,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
