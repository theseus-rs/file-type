use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856655: FileFormat = FileFormat {
    id: 105_856_655,
    puid: "wikidata/105856655",
    name: "Hourglass movie capture",
    extensions: &["hgm", "wtf"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x66, 0x54, 0x77, 0x02])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x66, 0x54, 0x77, 0x02])],
                },
            }],
        },
    ],
    related_formats: &[],
};
