use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_921122: FileType = FileType {
    file_format: &FileFormat {
        id: 921_122,
        source_type: SourceType::Wikidata,
        name: "Java archive",
        extensions: &["jar"],
        media_types: &["application/java-archive"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x5F, 0x27, 0xA8, 0x89])],
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
