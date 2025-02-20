use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854124: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_124,
        source_type: SourceType::Wikidata,
        name: "iPer Advanced Embedded Hypertext",
        extensions: &["aeh"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4C, 0x44, 0x4F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
