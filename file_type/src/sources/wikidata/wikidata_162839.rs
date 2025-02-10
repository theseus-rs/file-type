use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_162839: FileType = FileType {
    file_format: &FileFormat {
        id: 162_839,
        source_type: SourceType::Wikidata,
        name: "xz",
        extensions: &["xz"],
        media_types: &["application/x-xz"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x59, 0x5A])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
