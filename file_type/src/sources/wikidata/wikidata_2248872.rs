use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2248872: FileType = FileType {
    file_format: &FileFormat {
        id: 2_248_872,
        source_type: SourceType::Wikidata,
        name: "Open eBook",
        extensions: &["opf"],
        media_types: &["application/oebps-package+xml"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x56, 0x4E, 0x4D, 0x45, 0x42, 0x4F, 0x4F, 0x4B,
                        ])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                            0x6E, 0x3D,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
