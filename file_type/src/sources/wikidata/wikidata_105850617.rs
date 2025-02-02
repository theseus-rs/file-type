use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850617: FileFormat = FileFormat {
    id: 105_850_617,
    source_type: SourceType::Wikidata,
    name: "Montage Color data",
    extensions: &["color", "color1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x43, 0x4F, 0x4C, 0x4F, 0x52, 0x5F])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x43, 0x4F, 0x4C, 0x4F, 0x52, 0x5F])],
                },
            }],
        },
    ],
    related_formats: &[],
};
