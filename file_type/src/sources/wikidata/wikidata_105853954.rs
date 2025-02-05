use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853954: FileFormat = FileFormat {
    id: 105_853_954,
    source_type: SourceType::Wikidata,
    name: "PPMZ2 compressed data",
    extensions: &["ppmz2", "ppz2"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x70, 0x70, 0x7A, 0x32])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x70, 0x70, 0x7A, 0x32])],
                },
            }],
        },
    ],
    related_formats: &[],
};
