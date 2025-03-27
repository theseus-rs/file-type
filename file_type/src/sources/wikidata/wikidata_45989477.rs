use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_45989477: FileType = FileType {
    file_format: &FileFormat {
        id: 45_989_477,
        source_type: SourceType::Wikidata,
        name: "WebP Extended",
        extensions: &[],
        media_types: &["image/webp"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x57, 0x45, 0x42, 0x50, 0x56, 0x50, 0x38, 0x58]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
