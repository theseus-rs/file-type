use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3400889: FileFormat = FileFormat {
    id: 3_400_889,
    puid: "wikidata/3400889",
    name: "PowerPoint show",
    extensions: &["ppt", "ppt", "ppt"],
    media_types: &[
        "application/vnd.ms-powerpoint",
        "application/vnd.ms-powerpoint",
        "application/vnd.ms-powerpoint",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA0, 0x46, 0x1D, 0xF0])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0F, 0x00, 0xE8, 0x03])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x6E, 0x1E, 0xF0])],
                },
            }],
        },
    ],
    related_formats: &[],
};
