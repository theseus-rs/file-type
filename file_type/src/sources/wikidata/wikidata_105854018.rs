use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854018: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_018,
        source_type: SourceType::Wikidata,
        name: "AOP data",
        extensions: &["aop"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x41, 0x4F, 0x50, 0x31, 0x0D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
