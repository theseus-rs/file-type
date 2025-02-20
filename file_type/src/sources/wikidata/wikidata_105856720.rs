use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856720: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_720,
        source_type: SourceType::Wikidata,
        name: "Universal Data Format",
        extensions: &["udf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x24, 0x44, 0x4F, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
