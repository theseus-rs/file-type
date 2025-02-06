use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857065: FileFormat = FileFormat {
    id: 105_857_065,
    source_type: SourceType::Wikidata,
    name: "Vectrex game ROM",
    extensions: &["gam", "vec"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67, 0x20, 0x47, 0x43, 0x45, 0x20])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67, 0x20, 0x47, 0x43, 0x45, 0x20])],
                },
            }],
        },
    ],
    related_formats: &[],
};
