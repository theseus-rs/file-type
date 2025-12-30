use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3924: FileType = FileType {
    file_format: &FileFormat {
        id: 3_924,
        source_type: SourceType::Pronom,
        name: "ArcGIS Pro Layer File",
        extensions: &["lyrx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x7B]),
                        Token::WildcardCountRange(0, 1),
                        Token::Literal(&[
                            0x0A, 0x20, 0x20, 0x22, 0x74, 0x79, 0x70, 0x65, 0x22, 0x20, 0x3A, 0x20,
                            0x22, 0x43, 0x49, 0x4D, 0x4C, 0x61, 0x79, 0x65, 0x72, 0x44, 0x6F, 0x63,
                            0x75, 0x6D, 0x65, 0x6E, 0x74, 0x22, 0x2C,
                        ]),
                        Token::WildcardCountRange(0, 1),
                        Token::Literal(&[
                            0x0A, 0x20, 0x20, 0x22, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x22,
                            0x20, 0x3A, 0x20, 0x22,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
