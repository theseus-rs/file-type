use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862556: FileFormat = FileFormat {
    id: 105_862_556,
    source_type: SourceType::Wikidata,
    name: "MINC2 Medical Imaging format",
    extensions: &["mnc"],
    media_types: &["application/octet-stream", "application/x-minc"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x89, 0x48, 0x44, 0x46, 0x0D, 0x0A, 0x1A, 0x0A,
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
                        0x89, 0x48, 0x44, 0x46, 0x0D, 0x0A, 0x1A, 0x0A,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
