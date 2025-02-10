use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862023: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_023,
        source_type: SourceType::Wikidata,
        name: "Monarch for Windows Model",
        extensions: &["mod"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x46, 0x57, 0x4D, 0x4F, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
