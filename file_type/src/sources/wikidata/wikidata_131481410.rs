use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131481410: FileType = FileType {
    file_format: &FileFormat {
        id: 131_481_410,
        source_type: SourceType::Wikidata,
        name: "Free Lossless Audio Codec",
        extensions: &["flac"],
        media_types: &["audio/flac"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x66, 0x4C, 0x61, 0x43, 0x00, 0x00, 0x00, 0x22,
                        ])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x66, 0x4C, 0x61, 0x43])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
