use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859039: FileFormat = FileFormat {
    id: 105_859_039,
    source_type: SourceType::Wikidata,
    name: "DICOM medical imaging bitmap",
    extensions: &["dcm", "dic", "dicom"],
    media_types: &["application/dicom"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x49, 0x43, 0x4D])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x49, 0x43, 0x4D])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x49, 0x43, 0x4D])],
                },
            }],
        },
    ],
    related_formats: &[],
};
