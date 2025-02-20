use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852159: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_159,
        source_type: SourceType::Wikidata,
        name: "Visual Smalltalk Enterprise objects Library",
        extensions: &["sll"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x4F, 0x4C, 0x31, 0x30, 0x30, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
