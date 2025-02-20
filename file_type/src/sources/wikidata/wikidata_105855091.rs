use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855091: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_091,
        source_type: SourceType::Wikidata,
        name: "Mac AppleSingle encoded",
        extensions: &["as"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x05, 0x16, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
