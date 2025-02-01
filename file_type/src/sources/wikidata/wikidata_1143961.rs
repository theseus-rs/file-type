use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1143961: FileFormat = FileFormat {
    id: 1_143_961,
    puid: "wikidata/1143961",
    name: "JBIG2",
    extensions: &["jb2", "jbig2"],
    media_types: &["image/x-jbig2", "image/x-jbig2"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x97, 0x4A, 0x42, 0x32, 0x0D, 0x0A, 0x1A, 0x0A,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x97, 0x4A, 0x42, 0x32, 0x0D, 0x0A, 0x1A, 0x0A,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
