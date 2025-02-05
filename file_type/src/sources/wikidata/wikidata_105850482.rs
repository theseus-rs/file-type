use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850482: FileFormat = FileFormat {
    id: 105_850_482,
    source_type: SourceType::Wikidata,
    name: "CorelDraw compressed format (generic)",
    extensions: &["cdx", "cjw", "cpx"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x44, 0x52, 0x43, 0x4F, 0x4D, 0x50])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x44, 0x52, 0x43, 0x4F, 0x4D, 0x50])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x44, 0x52, 0x43, 0x4F, 0x4D, 0x50])],
                },
            }],
        },
    ],
    related_formats: &[],
};
