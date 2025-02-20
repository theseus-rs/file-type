use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852628: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_628,
        source_type: SourceType::Wikidata,
        name: "Scalable Vector Graphics (var.1)",
        extensions: &["svg"],
        media_types: &["image/svg+xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x73, 0x76, 0x67, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
