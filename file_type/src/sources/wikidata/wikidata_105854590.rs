use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854590: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_590,
        source_type: SourceType::Wikidata,
        name: "SJGPlay Album info",
        extensions: &["alb"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40, 0x41, 0x4C, 0x42, 0x55, 0x4D, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
