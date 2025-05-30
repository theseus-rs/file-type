use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959878: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_878,
        source_type: SourceType::Wikidata,
        name: "pxtone Collage module",
        extensions: &["ptcop", "pttune"],
        media_types: &["audio/x-mod"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x50, 0x54, 0x54, 0x55, 0x4E, 0x45, 0x2D, 0x2D, 0x32, 0x30, 0x30,
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
                            0x50, 0x54, 0x43, 0x4F, 0x4C, 0x4C, 0x41, 0x47, 0x45, 0x2D,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
