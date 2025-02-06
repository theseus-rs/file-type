use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858971: FileFormat = FileFormat {
    id: 105_858_971,
    source_type: SourceType::Wikidata,
    name: "PaintShop plus Compressed bitmap",
    extensions: &["da4", "psc"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x74, 0x6D, 0x38, 0x39, 0x50, 0x53])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x74, 0x6D, 0x38, 0x39, 0x50, 0x53])],
                },
            }],
        },
    ],
    related_formats: &[],
};
