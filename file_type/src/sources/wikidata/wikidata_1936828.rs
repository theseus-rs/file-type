use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1936828: FileFormat = FileFormat {
    id: 1_936_828,
    source_type: SourceType::Wikidata,
    name: "WAD",
    extensions: &["wad"],
    media_types: &["application/octet-stream", "application/wad"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x57, 0x41, 0x44])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x57, 0x41, 0x44])],
                },
            }],
        },
    ],
    related_formats: &[],
};
