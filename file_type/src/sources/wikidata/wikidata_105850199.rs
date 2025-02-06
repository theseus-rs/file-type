use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850199: FileFormat = FileFormat {
    id: 105_850_199,
    source_type: SourceType::Wikidata,
    name: "IBM Storyboard screen Capture",
    extensions: &["cap", "pic", "txm"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x50, 0x5F, 0x43, 0x41, 0x50])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x50, 0x5F, 0x43, 0x41, 0x50])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x50, 0x5F, 0x43, 0x41, 0x50])],
                },
            }],
        },
    ],
    related_formats: &[],
};
