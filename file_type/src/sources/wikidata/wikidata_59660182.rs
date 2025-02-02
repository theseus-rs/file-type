use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59660182: FileFormat = FileFormat {
    id: 59_660_182,
    source_type: SourceType::Wikidata,
    name: "qcow2",
    extensions: &["img", "qcow2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x51, 0x46, 0x49, 0xFB, 0x00, 0x00, 0x00, 0x02,
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
                        0x51, 0x46, 0x49, 0xFB, 0x00, 0x00, 0x00, 0x02,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
