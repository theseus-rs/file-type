use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864887: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_887,
        source_type: SourceType::Wikidata,
        name: "WinCC script",
        extensions: &["pas"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x50, 0x41, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
