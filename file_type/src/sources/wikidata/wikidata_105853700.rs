use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853700: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_700,
        source_type: SourceType::Wikidata,
        name: "EditPlus Auto Completion",
        extensions: &["acp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x54, 0x49, 0x54, 0x4C, 0x45, 0x3D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
