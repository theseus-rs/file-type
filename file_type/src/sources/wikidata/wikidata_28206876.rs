use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206876: FileFormat = FileFormat {
    id: 28_206_876,
    source_type: SourceType::Wikidata,
    name: "PCPaint PIC",
    extensions: &["clp", "pic"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFD, 0x00, 0xB8, 0x00, 0x00, 0x00, 0x40])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFD, 0x00, 0xB8, 0x00, 0x00, 0x00, 0x40])],
                },
            }],
        },
    ],
    related_formats: &[],
};
