use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_54809843: FileType = FileType {
    file_format: &FileFormat {
        id: 54_809_843,
        source_type: SourceType::Wikidata,
        name: "usdz file format",
        extensions: &["usdz"],
        media_types: &["application/octet-stream", "model/vnd.usdz+zip"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x50, 0x4B, 0x03, 0x04]),
                            Token::AnyWildcard,
                            Token::Literal(&[0x50, 0x58, 0x52, 0x2D, 0x55, 0x53, 0x44, 0x43]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
