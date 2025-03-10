use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859358: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_358,
        source_type: SourceType::Wikidata,
        name: "Apple QuickTake 100 photo",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x71, 0x6B, 0x74, 0x6B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
