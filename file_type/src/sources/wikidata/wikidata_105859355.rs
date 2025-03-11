use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859355: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_355,
        source_type: SourceType::Wikidata,
        name: "Apple QuickTake 150 photo",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x71, 0x6B, 0x74, 0x6E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
