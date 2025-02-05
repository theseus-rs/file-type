use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55721640: FileFormat = FileFormat {
    id: 55_721_640,
    source_type: SourceType::Wikidata,
    name: "AmiraMesh ASCII file format",
    extensions: &["am", "amiramesh", "hx"],
    media_types: &["text/plain"],
    signatures: &[
        Signature {
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
        Signature {
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
        Signature {
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
