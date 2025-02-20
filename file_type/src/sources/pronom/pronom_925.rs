use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_925: FileType = FileType {
    file_format: &FileFormat {
        id: 925,
        source_type: SourceType::Pronom,
        name: "Material Exchange Format",
        extensions: &["mxf"],
        media_types: &["application/mxf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x06, 0x0E, 0x2B, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0D, 0x01, 0x02, 0x01,
                            0x01, 0x02,
                        ]),
                        Token::WildcardCount(70),
                        Token::Literal(&[
                            0x06, 0x0E, 0x2B, 0x34, 0x04, 0x01, 0x01, 0x01, 0x0D, 0x01, 0x02, 0x01,
                            0x01, 0x01,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
